use crate::graphics::transform::{ModelMatrix, Transform};
use bytemuck::{Pod, Zeroable};
use cgmath::{Matrix4, SquareMatrix};

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct Uniforms {
    pub(crate) camera: [[f32; 4]; 4],
    pub(crate) transform: [[f32; 4]; 4],
    use_texture: u32,
    _padding: [[f32; 4]; 4], // Padding to ensure alignment
}

impl Uniforms {
    pub fn new() -> Self {
        Self {
            camera: Matrix4::identity().into(),
            transform: Matrix4::identity().into(),
            use_texture: 0,
            _padding: [[0.0; 4]; 4], // Padding to ensure alignment
        }
    }

    pub fn update_camera(&mut self, camera_matrix: [[f32; 4]; 4]) {
        self.camera = camera_matrix;
    }

    pub fn update_transform(&mut self, transform: &Transform) {
        self.transform = transform.to_matrix().into();
    }

    pub fn set_use_texture(&mut self, use_texture: bool) {
        self.use_texture = if use_texture { 1 } else { 0 };
    }
}