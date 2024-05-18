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

use wgpu::util::{BufferInitDescriptor, DeviceExt};

use crate::{
    math::linear_algebra::vec4::Vec4, render::renderer::Renderer,
    samplers::params::SAMPLES_PER_SEGMENT,
};

use super::{index_buffer_generator::IndexBufferGenerator, utils::create_span_buffer};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct SurfaceSamplerStage1Uniforms {
    control_count: u32,
    knot_count: u32,
    degree: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct SurfaceSamplerStage2Uniforms {
    control_count_u: u32,
    degree_u: u32,
    control_count_v: u32,
    degree_v: u32,
}

pub struct SurfaceSampler {
    renderer: Rc<Renderer>,
    bind_group_layout_stage_1: wgpu::BindGroupLayout,
    bind_group_layout_stage_2: wgpu::BindGroupLayout,
    pipeline_stage_1: wgpu::ComputePipeline,
    pipeline_stage_2: wgpu::ComputePipeline,
    index_buffer_generator: IndexBufferGenerator,
}

impl SurfaceSampler {
    pub fn new(renderer: Rc<Renderer>) -> SurfaceSampler {
        let device = renderer.get_device();
        let shader_module_stage_1 = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("surface sampler stage 1 compute shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("surface_sampler_stage_1.wgsl").into()),
        });
        let shader_module_stage_2 = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("surface sampler stage 2 compute shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("surface_sampler_stage_2.wgsl").into()),
        });
        let bind_group_layout_stage_1 =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
                    // Spans
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
                ],
            });

        let bind_group_layout_stage_2 =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
                    // Sapns U
                    wgpu::BindGroupLayoutEntry {
                        binding: 4,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    // Spans V
                    wgpu::BindGroupLayoutEntry {
                        binding: 5,
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
                        binding: 6,
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
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("surface sampler pipeline layout stage 1"),
                bind_group_layouts: &[&bind_group_layout_stage_1],
                push_constant_ranges: &[],
            });
        let pipeline_layout_stage_2 =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("surface sampler pipeline layout stage 2"),
                bind_group_layouts: &[&bind_group_layout_stage_2],
                push_constant_ranges: &[],
            });

        let pipeline_stage_1 = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("surface sampler pipeline"),
            layout: Some(&pipeline_layout_stage_1),
            module: &shader_module_stage_1,
            entry_point: "main",
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        });

        let pipeline_stage_2 = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("surface sampler pipeline"),
            layout: Some(&pipeline_layout_stage_2),
            module: &shader_module_stage_2,
            entry_point: "main",
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        });

        SurfaceSampler {
            renderer: renderer.clone(),
            bind_group_layout_stage_1,
            bind_group_layout_stage_2,
            pipeline_stage_1,
            pipeline_stage_2,
            index_buffer_generator: IndexBufferGenerator::new(device),
        }
    }

    fn create_basis_funcs(
        &self,
        control_count_u: u32,
        degree_u: u32,
        knots_u: &[f32],
        span_buffer_u: &wgpu::Buffer,
        control_count_v: u32,
        degree_v: u32,
        knots_v: &[f32],
        span_buffer_v: &wgpu::Buffer,
    ) -> (wgpu::Buffer, wgpu::Buffer) {
        let device = self.renderer.get_device();
        let queue = self.renderer.get_queue();

        let uniform_buffer_u = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("surface sampler stage 1 u uniform buffer"),
            contents: bytemuck::cast_slice(&[SurfaceSamplerStage1Uniforms {
                control_count: control_count_u,
                knot_count: knots_u.len() as u32,
                degree: degree_u,
            }]),
            usage: wgpu::BufferUsages::UNIFORM,
        });

        let uniform_buffer_v = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("surface sampler stage 1 v uniform buffer"),
            contents: bytemuck::cast_slice(&[SurfaceSamplerStage1Uniforms {
                control_count: control_count_v,
                knot_count: knots_v.len() as u32,
                degree: degree_v,
            }]),
            usage: wgpu::BufferUsages::UNIFORM,
        });

        let knot_buffer_u = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("surface sample knot u buffer"),
            contents: bytemuck::cast_slice(knots_u),
            usage: wgpu::BufferUsages::STORAGE,
        });
        let knot_buffer_v = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("surface sample knot v buffer"),
            contents: bytemuck::cast_slice(knots_v),
            usage: wgpu::BufferUsages::STORAGE,
        });

        let sample_count_u: u64 = SAMPLES_PER_SEGMENT as u64 * (control_count_u as u64 - 1) + 1;
        let sample_count_v: u64 = SAMPLES_PER_SEGMENT as u64 * (control_count_v as u64 - 1) + 1;

        let basis_funcs_u: wgpu::Buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("surface sampler basis funcs u buffer"),
            size: sample_count_u * (degree_u + 1) as u64 * std::mem::size_of::<f32>() as u64,
            usage: wgpu::BufferUsages::STORAGE,
            mapped_at_creation: false,
        });
        let basis_funcs_v: wgpu::Buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("surface sampler basis funcs v buffer"),
            size: sample_count_v * (degree_v + 1) as u64 * std::mem::size_of::<f32>() as u64,
            usage: wgpu::BufferUsages::STORAGE,
            mapped_at_creation: false,
        });

        let bind_group_u: wgpu::BindGroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
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
                    resource: span_buffer_u.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: basis_funcs_u.as_entire_binding(),
                },
            ],
        });

        let bind_group_v: wgpu::BindGroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
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
                    resource: span_buffer_v.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: basis_funcs_v.as_entire_binding(),
                },
            ],
        });

        let mut encoder_u = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("surface sampler stage 1 u command encoder"),
        });
        let mut encoder_v = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("surface sampler stage 1 v command encoder"),
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

            compute_pass_u.set_pipeline(&self.pipeline_stage_1);
            compute_pass_u.set_bind_group(0, &bind_group_u, &[]);
            compute_pass_u.dispatch_workgroups(sample_count_u as u32, 1, 1);

            compute_pass_v.set_pipeline(&self.pipeline_stage_1);
            compute_pass_v.set_bind_group(0, &bind_group_v, &[]);
            compute_pass_v.dispatch_workgroups(sample_count_v as u32, 1, 1);
        }

        let idx_u = queue.submit([encoder_u.finish()]);
        let idx_v = queue.submit([encoder_v.finish()]);

        device.poll(wgpu::Maintain::WaitForSubmissionIndex(idx_u));
        device.poll(wgpu::Maintain::WaitForSubmissionIndex(idx_v));

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
    ) -> (wgpu::Buffer, wgpu::Buffer) {
        let device = self.renderer.get_device();
        let queue = self.renderer.get_queue();

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("surface sampler stage 2 uniform buffer"),
            contents: bytemuck::cast_slice(&[SurfaceSamplerStage2Uniforms {
                control_count_u,
                degree_u,
                control_count_v,
                degree_v,
            }]),
            usage: wgpu::BufferUsages::UNIFORM,
        });

        let sample_count_u: u64 = SAMPLES_PER_SEGMENT as u64 * (control_count_u as u64 - 1) + 1;
        let sample_count_v: u64 = SAMPLES_PER_SEGMENT as u64 * (control_count_v as u64 - 1) + 1;

        let samples: wgpu::Buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("surface sampler output sample buffer"),
            size: sample_count_u * sample_count_v * 16 * 2,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        let vertex_buffer: wgpu::Buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("surface sampler output buffer"),
            size: sample_count_u * sample_count_v * 16 * 2,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let span_buffer_u = create_span_buffer(device, knots_u, degree_u, sample_count_u as u32);
        let span_buffer_v = create_span_buffer(device, knots_v, degree_v, sample_count_v as u32);

        let (basis_funcs_u, basis_funcs_v) = self.create_basis_funcs(
            control_count_u,
            degree_u,
            knots_u,
            &span_buffer_u,
            control_count_v,
            degree_v,
            knots_v,
            &span_buffer_v,
        );

        let control_point_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("surface sample control point buffer"),
            contents: bytemuck::cast_slice(weighted_controls),
            usage: wgpu::BufferUsages::STORAGE,
        });

        let bind_group: wgpu::BindGroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
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
                    resource: span_buffer_u.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 5,
                    resource: span_buffer_v.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 6,
                    resource: samples.as_entire_binding(),
                },
            ],
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
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
            &vertex_buffer,
            0,
            sample_count_u * sample_count_v * 16 * 2,
        );

        let idx = queue.submit([encoder.finish()]);

        device.poll(wgpu::Maintain::WaitForSubmissionIndex(idx));

        let index_buffer = self.index_buffer_generator.get_index_buffer(
            device,
            queue,
            sample_count_u as u32,
            sample_count_v as u32,
        );

        (index_buffer, vertex_buffer)
    }
}
