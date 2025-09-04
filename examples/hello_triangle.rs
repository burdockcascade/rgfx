use rgfx::prelude::*;

#[derive(Debug)]
pub struct MyWindow;

impl AppHandler for MyWindow {

    fn on_draw(&mut self, canvas: &mut Canvas) {

        canvas.clear(Color::WHITE);

        let camera = &mut Camera::default_orthographic(800.0, 600.0);

        // Draw a triangle
        let transform = Transform::new()
            .with_scale(0.75, 0.75, 0.75)
            .with_position(0.0, 0.0, 0.0);
        let draw_style = DrawStyle::default()
            .with_color(Color::BLUE);
        canvas.draw_triangle(camera, transform, draw_style);

    }

}

fn main() {
    Window::new(800, 600, "Hello Window", Box::new(MyWindow))
        .run();
}