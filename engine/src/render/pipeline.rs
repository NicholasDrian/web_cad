use crate::geometry::mesh::MESH_VERTEX_BUFFER_LAYOUT;

pub enum PipelinePrimitive {
    Mesh,
    Lines,
    Points,
}

pub fn create_render_pipeline(
    device: &wgpu::Device,
    format: &wgpu::TextureFormat,
    layouts: &[&wgpu::BindGroupLayout],
    shader_module: &wgpu::ShaderModule,
    primitive: PipelinePrimitive,
    samples: u32, // TODO:
) -> wgpu::RenderPipeline {
    let pipeline_layout: wgpu::PipelineLayout =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(match primitive {
                PipelinePrimitive::Mesh => "Triangle Pipeline Layout",
                PipelinePrimitive::Lines => "Curve Pipeline Layout",
                PipelinePrimitive::Points => "Surface Pipeline Layout",
            }),
            bind_group_layouts: layouts,
            push_constant_ranges: &[],
        });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(match primitive {
            PipelinePrimitive::Mesh => "Triangle Pipeline",
            PipelinePrimitive::Lines => "Curve Pipeline",
            PipelinePrimitive::Points => "Surface Pipeline",
        }),
        primitive: wgpu::PrimitiveState {
            topology: match primitive {
                PipelinePrimitive::Points => wgpu::PrimitiveTopology::PointList,
                PipelinePrimitive::Lines => wgpu::PrimitiveTopology::LineList,
                PipelinePrimitive::Mesh => wgpu::PrimitiveTopology::TriangleList,
            },
            strip_index_format: None,

            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        layout: Some(&pipeline_layout),
        depth_stencil: None,
        vertex: wgpu::VertexState {
            module: shader_module,
            entry_point: "vs_main",
            buffers: &[match primitive {
                PipelinePrimitive::Mesh => MESH_VERTEX_BUFFER_LAYOUT.clone(),
                PipelinePrimitive::Points => todo!(),
                PipelinePrimitive::Lines => todo!(),
            }][..],
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: shader_module,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: *format,
                blend: Some(wgpu::BlendState {
                    color: wgpu::BlendComponent::REPLACE,
                    alpha: wgpu::BlendComponent::REPLACE,
                }),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        }),
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    })
}
