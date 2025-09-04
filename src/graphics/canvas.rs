use crate::graphics::camera::CameraMatrix;
use crate::graphics::color::Color;
use crate::graphics::image::Image;
use crate::graphics::transform::Transform;
use crate::renderer::mesh::{Mesh, MeshBuilder2D};
use crate::renderer::state::DrawCommand;
use log::trace;

#[derive(Clone, Debug, Default)]
pub struct DrawStyle {
    pub color: Color,
    pub image: Option<Image>
}

impl DrawStyle {

    pub fn new(color: Color) -> Self {
        Self {
            color,
            image: None
        }
    }

    pub fn with_image(mut self, image: Image) -> Self {
        self.image = Some(image);
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct Canvas {
    draw_commands: Vec<DrawCommand>,
    pub(crate) bg_color: Color,
    pub width: f32,
    pub height: f32,
}

impl Canvas {

    pub fn clear(&mut self, bg_color: Color) {
        self.draw_commands.clear();
        self.bg_color = bg_color;
        trace!("Starting new frame");
    }

    /// Draws a triangle at the given transform with the given style.
    pub fn draw_triangle(&mut self, camera: &mut dyn CameraMatrix, transform: Transform, style: DrawStyle) {
        self.draw_mesh(camera, transform, MeshBuilder2D::from_triangle(style.color.into()), style);
    }

    /// Draws a rectangle at the given transform with the given style.
    pub fn draw_rectangle(&mut self, camera: &mut dyn CameraMatrix, transform: Transform, width: f32, height: f32, style: DrawStyle) {
        self.draw_mesh(camera, transform, MeshBuilder2D::from_rectangle(width, height, style.color.into()), style);
    }

    /// Draws a circle at the given transform with the given style.
    pub fn draw_circle(&mut self, camera: &mut dyn CameraMatrix, transform: Transform, radius: f32, style: DrawStyle) {
        self.draw_mesh(camera, transform, MeshBuilder2D::from_circle(radius, 32, style.color.into()), style);
    }

    /// Draws a mesh at the given transform with the given style.
    pub fn draw_mesh(&mut self, camera: &mut dyn CameraMatrix, transform: Transform, mesh: Mesh<crate::renderer::vertex::Vertex2D>, style: DrawStyle) {
        self.draw_commands.push(DrawCommand::DrawMesh2D {
            mesh,
            camera_matrix: camera.to_view_projection_matrix().into(),
            transform,
            style
        });
    }

    pub fn to_frame(&self) -> &[DrawCommand] {
        trace!("Getting frame with {} draw commands", self.draw_commands.len());
        self.draw_commands.as_slice()
    }

}


