use bytemuck::{Pod, Zeroable};
use wgpu::vertex_attr_array;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex2D {
    pub position: [f32; 2],
    pub uv: [f32; 2],
    pub color: [f32; 4]
}

impl Vertex2D {

    const ATTRIBUTES: [wgpu::VertexAttribute; 3] = vertex_attr_array![
        0 => Float32x2, // position
        1 => Float32x2,  // uv
        2 => Float32x4  // color
    ];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<Vertex2D>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex3D {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2]
}

impl Vertex3D {
    const ATTRIBUTES: [wgpu::VertexAttribute; 3] = vertex_attr_array![
        0 => Float32x3, // position
        1 => Float32x3, // normal
        2 => Float32x2  // uv
    ];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<Vertex3D>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES
        }
    }
}