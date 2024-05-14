//! Controls should be row major, and U major
//! The layout is as follows:
//!
//!     U ----->
//!   V 0, 1, 2,
//!   | 3, 4, 5,
//!   v 6, 7, 8,
//!
//!

use std::rc::Rc;

use crate::{
    math::linear_algebra::vec4::Vec4, render::renderer::Renderer,
    samplers::params::SAMPLES_PER_SEGMENT,
};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SurfaceSamplerUniforms {
    control_count_u: u32,
    knot_count_u: u32,
    degree_u: u32,
    control_count_v: u32,
    knot_count_v: u32,
    degree_v: u32,
}

pub struct SurfaceSampler {
    renderer: Rc<Renderer>,
    shader: wgpu::ShaderModule,
    bind_group_layout: wgpu::BindGroupLayout,
    uniform_buffer: wgpu::Buffer,
    pipeline: wgpu::ComputePipeline,
}

impl SurfaceSampler {
    pub fn new(renderer: Rc<Renderer>) -> SurfaceSampler {
        let shader = renderer
            .get_device()
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("curve sampler compute shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("curve_sampler.wgsl").into()),
            });

        let bind_group_layout =
            renderer
                .get_device()
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("curve sampler bind group layout"),

                    entries: &[
                        // Params
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                        // Controls
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage { read_only: true },
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                        // Knots
                        wgpu::BindGroupLayoutEntry {
                            binding: 2,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage { read_only: true },
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                        // Basis Funcs
                        wgpu::BindGroupLayoutEntry {
                            binding: 3,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage { read_only: false },
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                        // Samples
                        wgpu::BindGroupLayoutEntry {
                            binding: 4,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage { read_only: false },
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                    ],
                });

        let uniform_buffer = renderer
            .get_device()
            .create_buffer(&wgpu::BufferDescriptor {
                label: Some("curve sampler uniform buffer"),
                size: std::mem::size_of::<SurfaceSamplerUniforms>() as u64,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });

        let pipeline_layout =
            renderer
                .get_device()
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("curve sampler pipeline layout"),
                    bind_group_layouts: &[&bind_group_layout],
                    push_constant_ranges: &[],
                });

        let pipeline =
            renderer
                .get_device()
                .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    label: Some("curve sampler pipeline"),
                    layout: Some(&pipeline_layout),
                    module: &shader,
                    entry_point: "main",
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                });

        SurfaceSampler {
            renderer,
            shader,
            bind_group_layout,
            uniform_buffer,
            pipeline,
        }
    }

    fn create_basis_funcs() -> (wgpu::Buffer, wgpu::Buffer) {}

    pub async fn sample_curve(
        &self,
        degree_u: u32,
        degree_v: u32,
        weighted_controls: &[Vec4],
        control_count_u: u32,
        control_count_v: u32,
        knots_u: &[f32],
        knots_v: &[f32],
    ) -> wgpu::Buffer {
        self.renderer.get_queue().write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[SurfaceSamplerUniforms {
                control_count_u,
                knot_count_u: knots_u.len() as u32,
                degree_u,
                control_count_v,
                knot_count_v: knots_v.len() as u32,
                degree_v,
            }]),
        );

        let sample_count_u: u64 = SAMPLES_PER_SEGMENT as u64 * (control_count_u as u64 - 1) + 1;
        let sample_count_v: u64 = SAMPLES_PER_SEGMENT as u64 * (control_count_v as u64 - 1) + 1;

        let samples: wgpu::Buffer =
            self.renderer
                .get_device()
                .create_buffer(&wgpu::BufferDescriptor {
                    label: Some("curve sampler output sample buffer"),
                    size: sample_count_u * sample_count_v * 16,
                    usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
                    mapped_at_creation: false,
                });
        let output: wgpu::Buffer =
            self.renderer
                .get_device()
                .create_buffer(&wgpu::BufferDescriptor {
                    label: Some("curve sampler output buffer"),
                    size: sample_count_u * sample_count_v * 16,
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                });
        let basis_funcs_u: wgpu::Buffer =
            self.renderer
                .get_device()
                .create_buffer(&wgpu::BufferDescriptor {
                    label: Some("curve sampler basis funcs buffer"),
                    size: sample_count_u * (degree_u as u64 + 1) * 4,
                    usage: wgpu::BufferUsages::STORAGE,
                    mapped_at_creation: false,
                });
        let basis_funcs_v: wgpu::Buffer =
            self.renderer
                .get_device()
                .create_buffer(&wgpu::BufferDescriptor {
                    label: Some("curve sampler basis funcs buffer"),
                    size: sample_count_v * (degree_v as u64 + 1) * 4,
                    usage: wgpu::BufferUsages::STORAGE,
                    mapped_at_creation: false,
                });

        let control_point_buffer: wgpu::Buffer =
            self.renderer
                .get_device()
                .create_buffer(&wgpu::BufferDescriptor {
                    label: Some("curve sample control point buffer"),
                    size: weighted_controls.len() as u64 * std::mem::size_of::<Vec4>() as u64,
                    usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                });
        self.renderer.get_queue().write_buffer(
            &control_point_buffer,
            0,
            bytemuck::cast_slice(weighted_controls),
        );

        let knot_buffer: wgpu::Buffer =
            self.renderer
                .get_device()
                .create_buffer(&wgpu::BufferDescriptor {
                    label: Some("curve sample knot buffer"),
                    size: knots.len() as u64 * std::mem::size_of::<f32>() as u64,
                    usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                });
        self.renderer
            .get_queue()
            .write_buffer(&knot_buffer, 0, bytemuck::cast_slice(knots));

        let bind_group: wgpu::BindGroup =
            self.renderer
                .get_device()
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("curve sampler bind group"),
                    layout: &self.bind_group_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: self.uniform_buffer.as_entire_binding(),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: control_point_buffer.as_entire_binding(),
                        },
                        wgpu::BindGroupEntry {
                            binding: 2,
                            resource: knot_buffer.as_entire_binding(),
                        },
                        wgpu::BindGroupEntry {
                            binding: 3,
                            resource: basis_funcs.as_entire_binding(),
                        },
                        wgpu::BindGroupEntry {
                            binding: 4,
                            resource: samples.as_entire_binding(),
                        },
                    ],
                });

        let mut encoder =
            self.renderer
                .get_device()
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("curve sampler command encoder"),
                });

        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("curve sampler compute pass"),
                timestamp_writes: None,
            });

            compute_pass.set_pipeline(&self.pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);
            compute_pass.dispatch_workgroups(sample_count as u32, 1, 1);
        }

        encoder.copy_buffer_to_buffer(&samples, 0, &output, 0, sample_count * 16);

        let idx = self.renderer.get_queue().submit([encoder.finish()]);

        self.renderer
            .get_device()
            .poll(wgpu::Maintain::WaitForSubmissionIndex(idx));

        output
    }
}
