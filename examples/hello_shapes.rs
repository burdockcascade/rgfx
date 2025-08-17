use cgmath::{Point2, Vector2, Vector3};
use winit::keyboard::KeyCode;
use rgfx::prelude::*;

#[derive(Debug)]
pub struct MyWindow {
    camera: Camera,
    square_rotation: f32,
    rect_rotation: f32,
    rect_pos: Vector3<f32>,
    mouse_pos: Point2<f32>,
}

impl Default for MyWindow {
    fn default() -> Self {
        Self {
            camera: Camera::default_orthographic(800.0, 600.0),
            square_rotation: 0.0,
            rect_rotation: 0.0,
            rect_pos: Vector2::new(-0.4, 0.3).extend(0.0), // Initial position of the rectangle
            mouse_pos: Point2::new(0.0, 0.0),
        }
    }
}

impl AppHandler for MyWindow {

    fn on_event(&mut self, event: AppEvent) {
        match event {
            AppEvent::WindowResized(size) => {
                self.camera.update_viewport(size.x as f32, size.y as f32);
            }
            AppEvent::WindowClosed => {
                println!("Window closed");
            }
            AppEvent::KeyPressed(key) => {
                match key {
                    KeyCode::ArrowUp => self.camera.move_camera(CameraMovement::Up(0.1)),
                    KeyCode::ArrowDown => self.camera.move_camera(CameraMovement::Down(0.1)),
                    KeyCode::ArrowRight => self.camera.move_camera(CameraMovement::Right(0.1)),
                    KeyCode::ArrowLeft => self.camera.move_camera(CameraMovement::Left(0.1)),
                    KeyCode::KeyW => self.rect_pos.y += 0.1, // Move rectangle up
                    KeyCode::KeyS => self.rect_pos.y -= 0.1, // Move
                    KeyCode::KeyA => self.rect_pos.x -= 0.1, // Move rectangle left
                    KeyCode::KeyD => self.rect_pos.x += 0.1, // Move rectangle right
                    KeyCode::KeyQ => self.rect_rotation += 1.0, // Rotate rectangle clockwise
                    KeyCode::KeyE => self.rect_rotation -= 1.0, // Rotate rectangle counter-clockwise
                    _ => {}
                }

            }
            AppEvent::KeyReleased(key) => {
                //println!("Key released: {:?}", key);
            }
            AppEvent::CursorMoved(x, y) => {
                // Update mouse position
                self.mouse_pos = Point2::new(x as f32, y as f32);
            }
            AppEvent::MouseButtonPressed(button) => {
                //println!("Mouse button pressed: {}", button);
            }
            AppEvent::MouseButtonReleased(button) => {
                //println!("Mouse button released: {}", button);
            }
        }
    }

    fn on_update(&mut self, delta: f32) {

        // Update the square rotation based on the elapsed time
        self.square_rotation += delta * 50.0; // Rotate at 50 degrees per second

        // rotate the rectangle in reverse direction
        //self.rect_rotation -= delta * 10.0; // Rotate at 30 degrees per second

    }

    fn on_draw(&mut self, canvas: &mut Canvas) {

        canvas.clear(Color::WHITE);

        let camera = &mut self.camera;

        // Draw a rectangle
        let transform = Transform::new()
            .with_position_vector(self.rect_pos)
            .with_rotation(0.0, 0.0, self.rect_rotation);
        let draw_style = DrawStyle::default()
            .with_color(Color::RED);
        canvas.draw_rectangle(camera, transform, 1.0, 1.0,  draw_style);

        // Draw a triangle
        let transform = Transform::new()
            .with_position(0.0, 0.0, 0.0);
        let draw_style = DrawStyle::default()
            .with_color(Color::BLUE);
        canvas.draw_triangle(camera, transform, 0.0, draw_style);

        // Draw a circle
        let transform = Transform::new()
            .with_position(0.4, 0.0, 0.0);
        let draw_style = DrawStyle::default()
            .with_color(Color::GREEN);
        canvas.draw_circle(camera, transform, 0.5, draw_style);

        // Draw a square that rotates
        let transform = Transform::new()
            .with_position(-0.4, 0.0, 0.0)
            .with_rotation(0.0, 0.0, self.square_rotation);
        let draw_style = DrawStyle::default()
            .with_color(Color::YELLOW);
        canvas.draw_rectangle(camera, transform, 0.1, 0.5, draw_style);

    }

}

fn main() {
    Window::new(800, 600, "Hello Window", Box::new(MyWindow::default()))
        .run();
}