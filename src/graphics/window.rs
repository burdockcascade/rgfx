use log::{debug, error};
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};

pub struct Window {
    winit_window: Option<Arc<winit::window::Window>>,
    window_attributes: winit::window::WindowAttributes
}

impl Window {

    pub fn new(width : i32, height : i32, title : &str) -> Self {
        let window_attributes = winit::window::Window::default_attributes()
            .with_title(title)
            .with_inner_size(winit::dpi::PhysicalSize::new(width, height));

        Self {
            window_attributes,
            winit_window: None
        }
    }

    pub fn run(&mut self) {
        match EventLoop::new() {
            Ok(event_loop) => event_loop.run_app(self).expect("Unable to run app"),
            Err(e) => error!("Error creating event loop: {}", e)
        }
    }

}

impl ApplicationHandler for Window {

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {

        let window = match event_loop.create_window(self.window_attributes.clone()) {
            Ok(window) => Arc::new(window),
            Err(e) => {
                error!("Error creating window: {}", e);
                return;
            }
        };

        self.winit_window = Some(window.clone());

    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: winit::window::WindowId, event: WindowEvent) {

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                match event.physical_key {
                    PhysicalKey::Code(code) => {
                        if code == KeyCode::Escape {
                            event_loop.exit();
                        }
                    },
                    _ => {
                        debug!("Unhandled physical key: {:?}", event.physical_key);
                    }
                }
            }
            _ => {
                debug!("Unhandled window event: {:?}", event);
            }
        }

    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        match self.winit_window {
            Some(ref window) => {
                window.request_redraw();
            }
            None => {
                error!("No window found");
            }
        }
    }
}