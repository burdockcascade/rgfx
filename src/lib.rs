mod graphics;
mod renderer;
pub mod app;

pub mod prelude {
    pub use crate::app::*;
    pub use crate::graphics::camera::*;
    pub use crate::graphics::canvas::*;
    pub use crate::graphics::color::*;
    pub use crate::graphics::image::*;
    pub use crate::graphics::transform::*;
    pub use winit::keyboard::KeyCode;
    pub use cgmath::{Point2, Point3, Vector2, Vector3};
}