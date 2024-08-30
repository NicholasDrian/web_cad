use crate::geometry::{
    lines::LINES_VERTEX_BUFFER_LAYOUT, mesh::MESH_VERTEX_BUFFER_LAYOUT,
    polyline::POLYLINE_VERTEX_BUFFER_LAYOUT,
};

pub enum PipelinePrimitive {
    Mesh,
    LineStrip,
    Lines,
    Points,
}

pub fn create_render_pipeline(
    device: &wgpu::Device,
    layouts: &[&wgpu::BindGroupLayout],
    shader_module: &wgpu::ShaderModule,
    primitive: PipelinePrimitive,
    sample_count: u32,
) -> wgpu::RenderPipeline {
    let pipeline_layout: wgpu::PipelineLayout =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(match primitive {
                PipelinePrimitive::Mesh => "Triangle Pipeline Layout",
                PipelinePrimitive::LineStrip => "Line Strip Pipeline Layout",
                PipelinePrimitive::Lines => "Lines Layout",
                PipelinePrimitive::Points => "Surface Pipeline Layout",
            }),
            bind_group_layouts: layouts,
            push_constant_ranges: &[],
        });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        cache: None,
        label: Some(match primitive {
            PipelinePrimitive::Mesh => "Triangle Pipeline",
            PipelinePrimitive::Lines => "Lines Pipeline",
            PipelinePrimitive::Points => "Surface Pipeline",
            PipelinePrimitive::LineStrip => "Line Strip Pipeline",
        }),
        primitive: wgpu::PrimitiveState {
            topology: match primitive {
                PipelinePrimitive::Points => wgpu::PrimitiveTopology::PointList,
                PipelinePrimitive::Lines => wgpu::PrimitiveTopology::LineList,
                PipelinePrimitive::Mesh => wgpu::PrimitiveTopology::TriangleList,
                PipelinePrimitive::LineStrip => wgpu::PrimitiveTopology::LineStrip,
            },
            strip_index_format: None,

            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        layout: Some(&pipeline_layout),
        depth_stencil: Some(wgpu::DepthStencilState {
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState::default(),
            format: wgpu::TextureFormat::Depth24Plus,
            depth_write_enabled: true,
            bias: wgpu::DepthBiasState::default(),
        }),
        vertex: wgpu::VertexState {
            module: shader_module,
            entry_point: "vs_main",
            buffers: &[match primitive {
                PipelinePrimitive::Mesh => MESH_VERTEX_BUFFER_LAYOUT.clone(),
                PipelinePrimitive::LineStrip => POLYLINE_VERTEX_BUFFER_LAYOUT.clone(),
                PipelinePrimitive::Points => todo!(),
                PipelinePrimitive::Lines => LINES_VERTEX_BUFFER_LAYOUT.clone(),
            }][..],
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: shader_module,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Bgra8Unorm,
                blend: Some(wgpu::BlendState {
                    color: wgpu::BlendComponent::REPLACE,
                    alpha: wgpu::BlendComponent::REPLACE,
                }),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        }),
        multisample: wgpu::MultisampleState {
            count: sample_count,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    })
}
