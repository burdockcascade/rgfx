use crate::prelude::Canvas;
use crate::renderer::state::RenderState;
use cgmath::Vector2;
use log::{debug, error};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};

pub enum AppEvent {
    WindowResized(Vector2<u32>),
    WindowClosed,
    KeyPressed(KeyCode),
    KeyReleased(KeyCode),
    CursorMoved(f64, f64),
    MouseButtonPressed(u8),
    MouseButtonReleased(u8),
}

#[allow(unused)]
pub trait AppHandler {

    /// Called when the window is initialized.
    fn on_init(&mut self) {}

    fn on_event(&mut self, event: AppEvent) {
    }

    /// Called on each frame update.
    fn on_update(&mut self, delta: f32) {}

    /// Called on each frame draw.
    fn on_draw(&mut self, draw: &mut Canvas) {}

    /// Called when the window is about to close.
    fn on_close(&mut self) -> bool { true }
}

pub struct Window {
    winit_window: Option<Arc<winit::window::Window>>,
    canvas: Canvas,
    window_attributes: winit::window::WindowAttributes,
    display: Option<RenderState>,
    handler: Box<dyn AppHandler>,
    elapsed_since_last_frame: f32,
    start: std::time::Instant,
    target_frame_time: f32,
}

impl Window {

    pub fn new(width: u32, height: u32, title: &str, handler: Box<dyn AppHandler>) -> Self {
        let window_attributes = winit::window::Window::default_attributes()
            .with_title(title)
            .with_inner_size(winit::dpi::PhysicalSize::new(width, height));

        let draw_commander = Canvas::default();

        Self {
            window_attributes,
            canvas: draw_commander,
            display: None,
            winit_window: None,
            handler,
            elapsed_since_last_frame: 0.0,
            start: std::time::Instant::now(),
            target_frame_time: 1.0 / 60.0, // default to 60 FPS
        }
    }

    pub fn run(&mut self) {
        match EventLoop::new() {
            Ok(event_loop) => event_loop.run_app(self).expect("Unable to run app"),
            Err(e) => error!("Error creating event loop: {}", e)
        }
    }

    pub fn set_target_fps(&mut self, target: u8) -> &mut Self {
        self.target_frame_time = 1.0 / target as f32;
        self
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


        let display = RenderState::new(window.clone());

        //println!("Adaptor: {:?}", display.get_adaptor_info());

        self.display = Some(display);
        self.winit_window = Some(window.clone());

        self.handler.on_init();

    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: winit::window::WindowId, event: WindowEvent) {

        let Some(ref mut display) = self.display else {
            panic!("No display found");
        };

        match event {
            WindowEvent::RedrawRequested => {

                // start the frame timer
                self.start = std::time::Instant::now();

                // call the update handler
                self.handler.on_update(self.elapsed_since_last_frame);

                // call the draw handler
                self.handler.on_draw(&mut self.canvas);

                // render the frame
                display.render(&self.canvas);

                // sleep to reach target fps
                let sleep_time = self.target_frame_time - self.start.elapsed().as_secs_f32();
                if sleep_time > 0.0 {
                    thread::sleep(Duration::from_secs_f32(sleep_time));
                }

                self.elapsed_since_last_frame = self.start.elapsed().as_secs_f32();

            }
            WindowEvent::CloseRequested => {
                if !self.handler.on_close() {
                    return;
                }
                self.handler.on_event(AppEvent::WindowClosed);
                event_loop.exit();
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.handler.on_event(AppEvent::CursorMoved(position.x, position.y));
            }
            WindowEvent::KeyboardInput { event, .. } => {
                match event.physical_key {
                    PhysicalKey::Code(code) => {

                        self.handler.on_event(AppEvent::KeyPressed(code));

                        if code == KeyCode::Escape {
                            self.handler.on_event(AppEvent::WindowClosed);
                            event_loop.exit();
                        }

                    },
                    _ => {
                        debug!("Unhandled physical key: {:?}", event.physical_key);
                    }
                }
            }
            WindowEvent::Resized(physical_size) => {
                display.resize(physical_size);
                self.handler.on_event(AppEvent::WindowResized(Vector2::new(physical_size.width, physical_size.height)));
                self.canvas.width = physical_size.width as f32;
                self.canvas.height = physical_size.height as f32;
            }
            _ => {
                debug!("Unhandled window event: {:?}", event);
            }
        }

    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        match self.winit_window {
            Some(ref window) => {
                window.request_redraw();
            }
            None => {
                error!("Window not yet initialized");
                event_loop.exit();
            }
        }
    }
}