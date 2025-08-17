use cgmath::{Vector2, Vector3};
use winit::keyboard::KeyCode;
use rgfx::prelude::*;

#[derive(Debug)]
pub struct MyWindow {
    camera: Camera,
    square_rotation: f32,
    rect_rotation: f32,
    rect_pos: Vector3<f32>,
    tintin_image: Option<Image>,
}

impl Default for MyWindow {
    fn default() -> Self {
        Self {
            camera: Camera::default_orthographic(800.0, 600.0),
            square_rotation: 0.0,
            rect_rotation: 0.0,
            rect_pos: Vector2::new(-0.4, 0.3).extend(0.0), // Initial position of the rectangle
            tintin_image: None,
        }
    }
}

impl AppHandler for MyWindow {
    fn on_init(&mut self) {

        self.tintin_image = Some(Image::from_file("C:/Workspace/rgfx/examples/assets/tintin.jpg"));

    }

    fn on_event(&mut self, event: AppEvent) {
        match event {
            AppEvent::KeyPressed(key) => {
                match key {
                    KeyCode::ArrowUp => self.camera.move_camera(CameraMovement::Up(0.1)),
                    KeyCode::ArrowDown => self.camera.move_camera(CameraMovement::Down(0.1)),
                    KeyCode::ArrowRight => self.camera.move_camera(CameraMovement::Right(0.1)),
                    KeyCode::ArrowLeft => self.camera.move_camera(CameraMovement::Left(0.1)),
                    KeyCode::KeyW => self.rect_pos.y += 0.01, // Move rectangle up
                    KeyCode::KeyS => self.rect_pos.y -= 0.01, // Move
                    KeyCode::KeyA => self.rect_pos.x -= 0.01, // Move rectangle left
                    KeyCode::KeyD => self.rect_pos.x += 0.01, // Move rectangle right
                    KeyCode::KeyQ => self.rect_rotation += 1.0, // Rotate rectangle clockwise
                    KeyCode::KeyE => self.rect_rotation -= 1.0, // Rotate rectangle counter-clockwise
                    _ => println!("key: {:?}", key),
                }

            }
            _ => {
                // Handle other events if necessary
            }
        }
    }

    fn on_update(&mut self, delta: f32) {

        // Update the square rotation based on the elapsed time
        self.square_rotation += delta * 50.0; // Rotate at 50 degrees per second

    }

    fn on_draw(&mut self, canvas: &mut Canvas) {

        canvas.clear(Color::WHITE);

        let camera = &mut self.camera;

        // Draw a rectangle
        let transform = Transform::new()
            .with_position_vector(self.rect_pos)
            .with_rotation(0.0, 0.0, self.rect_rotation);

        let draw_style = DrawStyle::default()
            .with_image(self.tintin_image.clone().unwrap());

        canvas.draw_rectangle(camera, transform, 1.0, 1.0,  draw_style);

    }

}

fn main() {
    Window::new(800, 600, "Hello Window", Box::new(MyWindow::default()))
        .run();
}