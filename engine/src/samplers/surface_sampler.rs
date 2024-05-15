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
pub struct SurfaceSamplerStage1Uniforms {
    control_count: u32,
    knot_count: u32,
    degree: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SurfaceSamplerStage2Uniforms {
    control_count_u: u32,
    degree_u: u32,
    control_count_v: u32,
    degree_v: u32,
}

pub struct SurfaceSampler {
    renderer: Rc<Renderer>,
    shader_module_stage_1: wgpu::ShaderModule,
    shader_module_stage_2: wgpu::ShaderModule,
    bind_group_layout_stage_1: wgpu::BindGroupLayout,
    bind_group_layout_stage_2: wgpu::BindGroupLayout,
    pipeline_stage_1_u: wgpu::ComputePipeline,
    pipeline_stage_1_v: wgpu::ComputePipeline,
    pipeline_stage_2: wgpu::ComputePipeline,
}

impl SurfaceSampler {
    pub fn new(renderer: Rc<Renderer>) -> SurfaceSampler {
        let shader_module_stage_1 =
            renderer
                .get_device()
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("surface sampler stage 1 compute shader"),
                    source: wgpu::ShaderSource::Wgsl(
                        include_str!("surface_sampler_stage_1.wgsl").into(),
                    ),
                });
        let shader_module_stage_2 =
            renderer
                .get_device()
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("surface sampler stage 2 compute shader"),
                    source: wgpu::ShaderSource::Wgsl(
                        include_str!("surface_sampler_stage_2.wgsl").into(),
                    ),
                });
        let bind_group_layout_stage_1 =
            renderer
                .get_device()
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("surface sampler stage 2 bind group layout"),

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
                        // Knots
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
                        // Basis Funcs
                        wgpu::BindGroupLayoutEntry {
                            binding: 2,
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
        let bind_group_layout_stage_2 =
            renderer
                .get_device()
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("surface sampler stage 2 bind group layout"),

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
                        // Basis Funcs U
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
                        // Basis Funcs V
                        wgpu::BindGroupLayoutEntry {
                            binding: 3,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage { read_only: true },
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

        let pipeline_layout_stage_1 =
            renderer
                .get_device()
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("surface sampler pipeline layout stage 1"),
                    bind_group_layouts: &[&bind_group_layout_stage_1],
                    push_constant_ranges: &[],
                });
        let pipeline_layout_stage_2 =
            renderer
                .get_device()
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("surface sampler pipeline layout stage 2"),
                    bind_group_layouts: &[&bind_group_layout_stage_2],
                    push_constant_ranges: &[],
                });

        let pipeline_stage_1_u =
            renderer
                .get_device()
                .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    label: Some("surface sampler pipeline"),
                    layout: Some(&pipeline_layout_stage_1),
                    module: &shader_module_stage_1,
                    entry_point: "main",
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                });

        let pipeline_stage_1_v =
            renderer
                .get_device()
                .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    label: Some("surface sampler pipeline"),
                    layout: Some(&pipeline_layout_stage_1),
                    module: &shader_module_stage_1,
                    entry_point: "main",
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                });
        let pipeline_stage_2 =
            renderer
                .get_device()
                .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    label: Some("surface sampler pipeline"),
                    layout: Some(&pipeline_layout_stage_2),
                    module: &shader_module_stage_2,
                    entry_point: "main",
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                });

        SurfaceSampler {
            renderer,
            shader_module_stage_1,
            shader_module_stage_2,
            bind_group_layout_stage_1,
            bind_group_layout_stage_2,
            pipeline_stage_1_u,
            pipeline_stage_1_v,
            pipeline_stage_2,
        }
    }

    fn create_basis_funcs(
        &self,
        control_count_u: u32,
        degree_u: u32,
        control_count_v: u32,
        degree_v: u32,
        knots_u: &[f32],
        knots_v: &[f32],
    ) -> (wgpu::Buffer, wgpu::Buffer) {
        let uniform_buffer_u = self
            .renderer
            .get_device()
            .create_buffer(&wgpu::BufferDescriptor {
                label: Some("surface sampler stage 1 U uniform buffer"),
                size: std::mem::size_of::<SurfaceSamplerStage1Uniforms>() as u64,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        let uniform_buffer_v = self
            .renderer
            .get_device()
            .create_buffer(&wgpu::BufferDescriptor {
                label: Some("surface sampler stage 1 V uniform buffer"),
                size: std::mem::size_of::<SurfaceSamplerStage1Uniforms>() as u64,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        self.renderer.get_queue().write_buffer(
            &uniform_buffer_u,
            0,
            bytemuck::cast_slice(&[SurfaceSamplerStage1Uniforms {
                control_count: control_count_u,
                knot_count: knots_u.len() as u32,
                degree: degree_u,
            }]),
        );
        self.renderer.get_queue().write_buffer(
            &uniform_buffer_v,
            0,
            bytemuck::cast_slice(&[SurfaceSamplerStage1Uniforms {
                control_count: control_count_v,
                knot_count: knots_v.len() as u32,
                degree: degree_v,
            }]),
        );
        let knot_buffer_u: wgpu::Buffer =
            self.renderer
                .get_device()
                .create_buffer(&wgpu::BufferDescriptor {
                    label: Some("surface sample knot u buffer"),
                    size: knots_u.len() as u64 * std::mem::size_of::<f32>() as u64,
                    usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                });
        self.renderer
            .get_queue()
            .write_buffer(&knot_buffer_u, 0, bytemuck::cast_slice(knots_u));

        let knot_buffer_v: wgpu::Buffer =
            self.renderer
                .get_device()
                .create_buffer(&wgpu::BufferDescriptor {
                    label: Some("surface sample knot v buffer"),
                    size: knots_v.len() as u64 * std::mem::size_of::<f32>() as u64,
                    usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                });
        self.renderer
            .get_queue()
            .write_buffer(&knot_buffer_v, 0, bytemuck::cast_slice(knots_v));

        let sample_count_u: u64 = SAMPLES_PER_SEGMENT as u64 * (control_count_u as u64 - 1) + 1;
        let sample_count_v: u64 = SAMPLES_PER_SEGMENT as u64 * (control_count_v as u64 - 1) + 1;
        let basis_funcs_u: wgpu::Buffer =
            self.renderer
                .get_device()
                .create_buffer(&wgpu::BufferDescriptor {
                    label: Some("surface sampler basis funcs u buffer"),
                    size: sample_count_u * (degree_u as u64 + 1) * 4,
                    usage: wgpu::BufferUsages::STORAGE,
                    mapped_at_creation: false,
                });
        let basis_funcs_v: wgpu::Buffer =
            self.renderer
                .get_device()
                .create_buffer(&wgpu::BufferDescriptor {
                    label: Some("surface sampler basis funcs v buffer"),
                    size: sample_count_v * (degree_v as u64 + 1) * 4,
                    usage: wgpu::BufferUsages::STORAGE,
                    mapped_at_creation: false,
                });

        let bind_group_u: wgpu::BindGroup =
            self.renderer
                .get_device()
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("surface sampler bind group"),
                    layout: &self.bind_group_layout_stage_1,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: uniform_buffer_u.as_entire_binding(),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: knot_buffer_u.as_entire_binding(),
                        },
                        wgpu::BindGroupEntry {
                            binding: 2,
                            resource: basis_funcs_u.as_entire_binding(),
                        },
                    ],
                });

        let bind_group_v: wgpu::BindGroup =
            self.renderer
                .get_device()
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("surface sampler bind group"),
                    layout: &self.bind_group_layout_stage_1,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: uniform_buffer_v.as_entire_binding(),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: knot_buffer_v.as_entire_binding(),
                        },
                        wgpu::BindGroupEntry {
                            binding: 2,
                            resource: basis_funcs_v.as_entire_binding(),
                        },
                    ],
                });

        let mut encoder_u =
            self.renderer
                .get_device()
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("surface sampler stage 1 command encoder"),
                });
        let mut encoder_v =
            self.renderer
                .get_device()
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("surface sampler stage 1 command encoder"),
                });

        {
            let mut compute_pass_u = encoder_u.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("surface sampler stage 1 u compute pass"),
                timestamp_writes: None,
            });
            let mut compute_pass_v = encoder_v.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("surface sampler stage 1 u compute pass"),
                timestamp_writes: None,
            });

            compute_pass_u.set_pipeline(&self.pipeline_stage_1_u);
            compute_pass_u.set_bind_group(0, &bind_group_u, &[]);
            compute_pass_u.dispatch_workgroups(sample_count_u as u32, sample_count_v as u32, 1);

            compute_pass_v.set_pipeline(&self.pipeline_stage_1_v);
            compute_pass_v.set_bind_group(0, &bind_group_v, &[]);
            compute_pass_v.dispatch_workgroups(sample_count_u as u32, sample_count_v as u32, 1);
        }

        let idx_u = self.renderer.get_queue().submit([encoder_u.finish()]);
        let idx_v = self.renderer.get_queue().submit([encoder_v.finish()]);

        self.renderer
            .get_device()
            .poll(wgpu::Maintain::WaitForSubmissionIndex(idx_u));
        self.renderer
            .get_device()
            .poll(wgpu::Maintain::WaitForSubmissionIndex(idx_v));

        (basis_funcs_u, basis_funcs_v)
    }

    pub fn sample_surface(
        &self,
        degree_u: u32,
        degree_v: u32,
        weighted_controls: &[Vec4],
        control_count_u: u32,
        control_count_v: u32,
        knots_u: &[f32],
        knots_v: &[f32],
    ) -> wgpu::Buffer {
        let uniform_buffer = self
            .renderer
            .get_device()
            .create_buffer(&wgpu::BufferDescriptor {
                label: Some("surface sampler stage 2 uniform buffer"),
                size: std::mem::size_of::<SurfaceSamplerStage2Uniforms>() as u64,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });

        self.renderer.get_queue().write_buffer(
            &uniform_buffer,
            0,
            bytemuck::cast_slice(&[SurfaceSamplerStage2Uniforms {
                control_count_u,
                degree_u,
                control_count_v,
                degree_v,
            }]),
        );

        let sample_count_u: u64 = SAMPLES_PER_SEGMENT as u64 * (control_count_u as u64 - 1) + 1;
        let sample_count_v: u64 = SAMPLES_PER_SEGMENT as u64 * (control_count_v as u64 - 1) + 1;

        let samples: wgpu::Buffer =
            self.renderer
                .get_device()
                .create_buffer(&wgpu::BufferDescriptor {
                    label: Some("surface sampler output sample buffer"),
                    size: sample_count_u * sample_count_v * 16,
                    usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
                    mapped_at_creation: false,
                });
        let output: wgpu::Buffer =
            self.renderer
                .get_device()
                .create_buffer(&wgpu::BufferDescriptor {
                    label: Some("surface sampler output buffer"),
                    size: sample_count_u * sample_count_v * 16,
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                });

        let (basis_funcs_u, basis_funcs_v) = self.create_basis_funcs(
            control_count_v,
            degree_u,
            control_count_v,
            degree_v,
            knots_u,
            knots_v,
        );

        let control_point_buffer: wgpu::Buffer =
            self.renderer
                .get_device()
                .create_buffer(&wgpu::BufferDescriptor {
                    label: Some("surface sample control point buffer"),
                    size: weighted_controls.len() as u64 * std::mem::size_of::<Vec4>() as u64,
                    usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                });
        self.renderer.get_queue().write_buffer(
            &control_point_buffer,
            0,
            bytemuck::cast_slice(weighted_controls),
        );

        let bind_group: wgpu::BindGroup =
            self.renderer
                .get_device()
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("surface sampler bind group"),
                    layout: &self.bind_group_layout_stage_2,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: uniform_buffer.as_entire_binding(),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: control_point_buffer.as_entire_binding(),
                        },
                        wgpu::BindGroupEntry {
                            binding: 2,
                            resource: basis_funcs_u.as_entire_binding(),
                        },
                        wgpu::BindGroupEntry {
                            binding: 3,
                            resource: basis_funcs_v.as_entire_binding(),
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
                    label: Some("surface sampler stage 2 command encoder"),
                });

        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("surface sampler stage 2 compute pass"),
                timestamp_writes: None,
            });

            compute_pass.set_pipeline(&self.pipeline_stage_2);
            compute_pass.set_bind_group(0, &bind_group, &[]);
            compute_pass.dispatch_workgroups(sample_count_u as u32, sample_count_v as u32, 1);
        }

        encoder.copy_buffer_to_buffer(
            &samples,
            0,
            &output,
            0,
            sample_count_u * sample_count_v * 16,
        );

        let idx = self.renderer.get_queue().submit([encoder.finish()]);

        self.renderer
            .get_device()
            .poll(wgpu::Maintain::WaitForSubmissionIndex(idx));

        output
    }
}
