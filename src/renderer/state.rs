use crate::graphics::color::Color;
use crate::graphics::image::Image;
use crate::graphics::transform::Transform;
use crate::prelude::{Canvas, DrawStyle};
use crate::renderer::pipeline::PipelineBuilder;
use crate::renderer::texture::Texture;
use crate::renderer::uniform::Uniform2D;
use crate::renderer::vertex::Vertex2D;
use crate::renderer::mesh::Mesh;
use pollster::FutureExt;
use std::cmp::max;
use std::sync::Arc;
use std::collections::HashMap;
use wgpu::util::DeviceExt;
use wgpu::{Adapter, AdapterInfo, BindGroup, BindGroupLayout, Buffer, Device, Instance, PresentMode, Queue, Surface};
use winit::dpi::PhysicalSize;
use winit::window::Window;

const MAX_INSTANCES: usize = 1000; //

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
    uniform_buffer: Buffer,
    uniform_bind_group: BindGroup,
    texture_bind_group_layout: BindGroupLayout,
    texture_cache: HashMap<String, (Texture, BindGroup)>,
    default_white_texture: (Texture, BindGroup),
    padded_uniform_size: u64, 
}

impl RenderState {
    pub fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();
        let instance = Instance::new(&wgpu::InstanceDescriptor::default());

        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default()).block_on().unwrap();
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default()).block_on().unwrap();

        // Calculate alignment
        let alignment = device.limits().min_uniform_buffer_offset_alignment as u64;
        let uniform_size = std::mem::size_of::<Uniform2D>() as u64;
        let padded_uniform_size = (uniform_size + alignment - 1) & !(alignment - 1);

        let surface = instance.create_surface(window).unwrap();
        let surface_caps = surface.get_capabilities(&adapter);
        let format = surface_caps.formats.iter().find(|f| f.is_srgb()).copied().unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width,
            height: size.height,
            present_mode: PresentMode::AutoNoVsync,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let uniform_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: true,
                    min_binding_size: wgpu::BufferSize::new(uniform_size),
                },
                count: None,
            }],
            label: Some("uniform_layout"),
        });

        let texture_layout = Self::create_texture_bind_group_layout(&device);

        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Multi-Instance Uniform Buffer"),
            size: padded_uniform_size * MAX_INSTANCES as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &uniform_buffer,
                    offset: 0,
                    size: wgpu::BufferSize::new(uniform_size),
                }),
            }],
            label: Some("uniform_bind_group"),
        });

        let white_pixel = Image::single_pixel(Color::WHITE);
        let white_tex = Texture::from_image(&device, white_pixel.image.clone());
        let (width, height) = white_tex.image.dimensions();
        queue.write_texture(
            wgpu::TexelCopyTextureInfo { texture: &white_tex.texture, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: Default::default() },
            &white_tex.image,
            wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(4 * width), rows_per_image: Some(height) },
            wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
        );

        let white_bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&white_tex.view) },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&white_tex.sampler) },
            ],
            label: None,
        });

        let shader = device.create_shader_module(wgpu::include_wgsl!("../shaders/shader2d.wgsl"));
        let render_pipeline = PipelineBuilder::new()
            .with_vertex_shader(&shader)
            .with_fragment_shader(&shader)
            .with_vertex_buffer_layout(Vertex2D::desc())
            .build(&device, config.format, &[&uniform_layout, &texture_layout])
            .expect("Failed to create pipeline");

        Self {
            surface, adapter, device, queue, config, size,
            render_pipeline,
            uniform2d: Uniform2D::new(),
            uniform_buffer,
            uniform_bind_group,
            texture_bind_group_layout: texture_layout,
            texture_cache: HashMap::new(),
            default_white_texture: (white_tex, white_bg),
            padded_uniform_size,
        }
    }

    pub fn render(&mut self, canvas: &Canvas) {
        let output = self.surface.get_current_texture().expect("Failed to get surface texture");
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let draw_commands = canvas.to_frame();
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

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
                ..Default::default()
            });

            render_pass.set_pipeline(&self.render_pipeline);

            for (i, command) in draw_commands.iter().enumerate() {
                if i >= MAX_INSTANCES { break; }

                match command {
                    DrawCommand::DrawMesh2D { mesh, camera_matrix, transform, style } => {
                        self.uniform2d.update_camera(*camera_matrix);
                        self.uniform2d.update_transform(transform);
                        self.uniform2d.set_use_texture(style.image.is_some());

                        // Use the padded offset
                        let offset = i as u64 * self.padded_uniform_size;
                        self.queue.write_buffer(&self.uniform_buffer, offset, bytemuck::cast_slice(&[self.uniform2d]));

                        render_pass.set_bind_group(0, &self.uniform_bind_group, &[offset as u32]);

                        let bind_group_1 = if let Some(img) = &style.image {
                            let device = &self.device;
                            let queue = &self.queue;
                            let layout = &self.texture_bind_group_layout;
                            let entry = self.texture_cache.entry(img.path.clone()).or_insert_with(|| {
                                let tex = Texture::from_image(device, img.image.clone());
                                let (w, h) = tex.image.dimensions();
                                queue.write_texture(
                                    wgpu::TexelCopyTextureInfo { texture: &tex.texture, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: Default::default() },
                                    &tex.image,
                                    wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(4 * w), rows_per_image: Some(h) },
                                    wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
                                );
                                let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
                                    layout,
                                    entries: &[
                                        wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&tex.view) },
                                        wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&tex.sampler) },
                                    ],
                                    label: None,
                                });
                                (tex, bg)
                            });
                            &entry.1
                        } else {
                            &self.default_white_texture.1
                        };

                        render_pass.set_bind_group(1, bind_group_1, &[]);

                        let v_buf = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                            label: None, contents: bytemuck::cast_slice(&mesh.vertices), usage: wgpu::BufferUsages::VERTEX,
                        });
                        let i_buf = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                            label: None, contents: bytemuck::cast_slice(&mesh.indices), usage: wgpu::BufferUsages::INDEX,
                        });

                        render_pass.set_vertex_buffer(0, v_buf.slice(..));
                        render_pass.set_index_buffer(i_buf.slice(..), wgpu::IndexFormat::Uint16);
                        render_pass.draw_indexed(0..mesh.indices.len() as u32, 0, 0..1);
                    }
                }
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
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

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.size = new_size;
        self.config.width = max(new_size.width, 1);
        self.config.height = max(new_size.height, 1);
        self.surface.configure(&self.device, &self.config);
    }
}