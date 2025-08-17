use crate::graphics::camera::CameraMatrix;
use crate::graphics::color::Color;
use crate::graphics::image::Image;
use crate::graphics::transform::Transform;
use crate::renderer::mesh::Mesh;
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

    pub fn draw_triangle(&mut self, camera: &mut dyn CameraMatrix, transform: Transform, size: f32, style: DrawStyle) {
        let mesh = Mesh::from_triangle(style.color.into());
        self.draw_mesh(mesh, camera, transform, style.image.clone());
    }

    pub fn draw_rectangle(&mut self, camera: &mut dyn CameraMatrix, transform: Transform, width: f32, height: f32, style: DrawStyle) {
        let mesh = Mesh::from_rectangle(width, height, style.color.into());
        self.draw_mesh(mesh, camera, transform, style.image.clone());
    }

    pub fn draw_circle(&mut self, camera: &mut dyn CameraMatrix, transform: Transform, radius: f32, style: DrawStyle) {
        let mesh = Mesh::from_circle(radius, 64, style.color.into());
        self.draw_mesh(mesh, camera, transform, style.image.clone());
    }

    pub fn draw_mesh(&mut self, mesh: Mesh, camera: &mut dyn CameraMatrix, transform: Transform, image: Option<Image>) {
        self.draw_commands.push(DrawCommand::DrawMesh2D {
            vertices: mesh.vertices,
            indices: mesh.indices,
            camera_matrix: camera.to_view_projection_matrix().into(),
            transform,
            image,
        });
    }

    pub fn to_frame(&self) -> &[DrawCommand] {
        trace!("Getting frame with {} draw commands", self.draw_commands.len());
        self.draw_commands.as_slice()
    }

}


