use wgpu::{BindGroupLayout, Device, PrimitiveTopology, ShaderModule, TextureFormat, VertexBufferLayout};

pub struct PipelineBuilder<'a> {
    label: Option<&'a str>,
    vertex_shader: Option<&'a ShaderModule>,
    fragment_shader: Option<&'a ShaderModule>,
    vertex_buffer_layouts: Vec<VertexBufferLayout<'a>>,
    primitive_topology: PrimitiveTopology,
}

impl<'a> PipelineBuilder<'a> {

    pub fn new() -> Self {
        Self {
            label: None,
            vertex_shader: None,
            fragment_shader: None,
            vertex_buffer_layouts: Vec::new(),
            primitive_topology: PrimitiveTopology::TriangleList,
        }
    }

    pub fn with_label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    pub fn with_vertex_shader(mut self, shader_module: &'a ShaderModule) -> Self {
        self.vertex_shader = Some(shader_module);
        self
    }

    pub fn with_fragment_shader(mut self, shader_module: &'a ShaderModule) -> Self {
        self.fragment_shader = Some(shader_module);
        self
    }

    pub fn with_vertex_buffer_layout(mut self, layout: VertexBufferLayout<'a>) -> Self {
        self.vertex_buffer_layouts.push(layout);
        self
    }

    pub fn with_primitive_topology(mut self, topology: PrimitiveTopology) -> Self {
        self.primitive_topology = topology;
        self
    }


    pub fn build(self, device: &Device, surface_format: TextureFormat, bind_group_layouts: &[&BindGroupLayout]) -> Result<wgpu::RenderPipeline, String> {

        let vertex_shader = self.vertex_shader.ok_or("Vertex shader must be provided")?;
        let fragment_shader = self.fragment_shader.ok_or("Fragment shader must be provided")?;

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: self.label.map(|l| format!("{} Layout", l)).as_deref(),
            bind_group_layouts, // Use the provided bind group layouts
            push_constant_ranges: &[], // No push constants for now
        });

        Ok(device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vertex_shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &self.vertex_buffer_layouts,
            },
            fragment: Some(wgpu::FragmentState {
                module: &fragment_shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
            primitive: wgpu::PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Cw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            cache: None,
        }))

    }

}