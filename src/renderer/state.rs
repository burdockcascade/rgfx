use crate::graphics::color::Color;
use crate::graphics::image::Image;
use crate::graphics::transform::Transform;
use crate::prelude::{Canvas, DrawStyle};
use crate::renderer::pipeline::PipelineBuilder;
use crate::renderer::texture::Texture;
use crate::renderer::uniform::Uniform2D;
use crate::renderer::vertex::Vertex2D;
use log::warn;
use pollster::FutureExt;
use std::cmp::max;
use std::sync::Arc;
use wgpu::util::DeviceExt;
use wgpu::{Adapter, AdapterInfo, BindGroup, BindGroupLayout, Buffer, Device, Instance, PresentMode, Queue, Surface};
use winit::dpi::PhysicalSize;
use winit::window::Window;
use crate::renderer::mesh::Mesh;

#[derive(Clone, Debug)]
pub enum DrawCommand {
    DrawMesh2D {
        mesh: Mesh<Vertex2D>,
        camera_matrix: [[f32; 4]; 4],
        transform: Transform,
        style: DrawStyle
    }
}

#[derive(Debug)]
pub struct RenderState {
    surface: Surface<'static>,
    adapter: Adapter,
    device: Device,
    queue: Queue,
    config: wgpu::SurfaceConfiguration,
    render_pipeline: wgpu::RenderPipeline,
    size: PhysicalSize<u32>,
    uniform2d: Uniform2D,
}

impl RenderState {
    pub fn new(window: Arc<Window>) -> Self {

        // Get the window size
        let size = window.inner_size();

        // Create a new instance
        let instance = Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // Request an adapter from the instance
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .block_on()
            .unwrap();

        // Create a new device and queue
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .block_on()
            .unwrap();

        // Create a new surface
        let surface = instance.create_surface(window).unwrap();

        // Get the capabilities of the surface
        let surface_caps = surface.get_capabilities(&adapter);

        // Create a new surface configuration
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_caps
                .formats
                .iter()
                .find(|f| f.is_srgb())
                .copied()
                .unwrap_or(surface_caps.formats[0]),
            width: size.width,
            height: size.height,
            present_mode: PresentMode::AutoNoVsync,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let shader = device.create_shader_module(wgpu::include_wgsl!("../shaders/shader2d.wgsl"));
        let render_pipeline = PipelineBuilder::new()
            .with_label("Render Pipeline")
            .with_vertex_shader(&shader)
            .with_fragment_shader(&shader)
            .with_vertex_buffer_layout(Vertex2D::desc())
            .with_primitive_topology(wgpu::PrimitiveTopology::TriangleList)
            .build(&device, config.format, &[
                &Self::create_uniform_bind_layout(&device),
                &Self::create_texture_bind_group_layout(&device),
            ])
            .expect("Failed to create render pipeline");

        Self {
            surface,
            adapter,
            device,
            queue,
            config,
            size,
            render_pipeline,
            uniform2d: Uniform2D::new(),
        }
    }

    pub fn get_adaptor_info(&self) -> AdapterInfo {
        self.adapter.get_info()
    }

    fn create_uniform_bind_layout(device: &Device) -> BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
            label: Some("uniform_bind_group_layout"),
        })
    }

    fn create_uniform_bind_group(device: &Device, uniforms: Uniform2D) -> BindGroup {
        let uniform_bind_group_layout = RenderState::create_uniform_bind_layout(&device);
        let transform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniforms Buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: transform_buffer.as_entire_binding(),
                },
            ],
            label: Some("transform_bind_group"),
        })
    }

    fn create_texture_bind_group_layout(device: &Device) -> BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("texture_bind_group_layout"),
        })
    }

    fn create_texture_bind_group(device: &Device, texture: &Texture) -> BindGroup {
        let texture_bind_group_layout = RenderState::create_texture_bind_group_layout(&device);
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                }
            ],
            label: Some("texture_bind_group"),
        })
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.size = new_size;
        self.config.width = max(new_size.width, 1);
        self.config.height = max(new_size.height, 1);
        self.surface.configure(&self.device, &self.config);
    }

    fn write_texture_to_queue(queue: &Queue, texture: &Texture) {
        let (width, height) = texture.image.dimensions();
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: Default::default(),
            },
            &texture.image,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );
    }

    fn create_vertex_buffer(&mut self, vertices: &[Vertex2D]) -> Buffer {
        self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        })
    }

    fn create_index_buffer(&mut self, indices: &[u16]) -> Buffer {
        self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        })
    }

    pub fn render(&mut self, canvas: &Canvas) {

        let output = match self.surface.get_current_texture() {
            Ok(o) => o,
            Err(e) => {
                warn!("Unable to get current texture: {:?}", e);
                return;
            }
        };

        let draw_commands = canvas.to_frame();

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(canvas.bg_color.into()),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
                multiview_mask: None,
            });

            for command in draw_commands.iter() {
                match command {
                    DrawCommand::DrawMesh2D { mesh, camera_matrix, transform, style } => {

                        let vertices = &mesh.vertices;
                        let indices = &mesh.indices;

                        render_pass.set_pipeline(&self.render_pipeline);
                        self.uniform2d.update_camera(*camera_matrix);
                        self.uniform2d.update_transform(transform);
                        self.uniform2d.set_use_texture(style.image.is_some());

                        // Group 0
                        let bind_group = Self::create_uniform_bind_group(&self.device, self.uniform2d);
                        render_pass.set_bind_group(0, &bind_group, &[]);

                        // Group 1
                        let texture = match &style.image {
                            Some(img) => {
                                Texture::from_image(&self.device, img.clone().image)
                            },
                            None => {
                                Texture::from_image(&self.device, Image::single_pixel(Color::WHITE).image.clone())
                            }
                        };
                        Self::write_texture_to_queue(&self.queue, &texture);
                        let bg = Self::create_texture_bind_group(&self.device, &texture);
                        render_pass.set_bind_group(1, &bg, &[]);

                        // Set vertex and index buffers
                        render_pass.set_vertex_buffer(0, self.create_vertex_buffer(&vertices).slice(..));
                        render_pass.set_index_buffer(self.create_index_buffer(&indices).slice(..), wgpu::IndexFormat::Uint16);
                        render_pass.draw_indexed(0..indices.len() as u32, 0, 0..1)

                    }
                }

            }

        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

    }

}