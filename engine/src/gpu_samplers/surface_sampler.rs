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
    gpu_samplers::params::SAMPLES_PER_SEGMENT, math::linear_algebra::vec4::Vec4,
    render::renderer::Renderer, utils::create_compute_pipeline,
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
    bind_group_layout_stage_3: wgpu::BindGroupLayout,
    pipeline_stage_1: wgpu::ComputePipeline,
    pipeline_stage_2: wgpu::ComputePipeline,
    pipeline_stage_3: wgpu::ComputePipeline,
    index_buffer_generator: IndexBufferGenerator,
}

impl SurfaceSampler {
    pub fn new(renderer: Rc<Renderer>) -> SurfaceSampler {
        let device = renderer.get_device();
        let bind_group_layout_stage_1 =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("surface sampler stage 2 bind group layout"),
                entries: &[
                    // Params
                    crate::utils::compute_uniform_bind_group_layout_entry(0),
                    // Knots
                    crate::utils::compute_buffer_bind_group_layout_entry(1, true),
                    // Spans
                    crate::utils::compute_buffer_bind_group_layout_entry(2, true),
                    // Basis Funcs
                    crate::utils::compute_buffer_bind_group_layout_entry(3, false),
                ],
            });

        let bind_group_layout_stage_2 =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("surface sampler stage 2 bind group layout"),

                entries: &[
                    // Params
                    crate::utils::compute_uniform_bind_group_layout_entry(0),
                    // Controls
                    crate::utils::compute_buffer_bind_group_layout_entry(1, true),
                    // Basis Funcs U
                    crate::utils::compute_buffer_bind_group_layout_entry(2, true),
                    // Basis Funcs V
                    crate::utils::compute_buffer_bind_group_layout_entry(3, true),
                    // Sapns U
                    crate::utils::compute_buffer_bind_group_layout_entry(4, true),
                    // Spans V
                    crate::utils::compute_buffer_bind_group_layout_entry(5, true),
                    // Samples
                    crate::utils::compute_buffer_bind_group_layout_entry(6, false),
                ],
            });
        let bind_group_layout_stage_3 =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("surface sampler stage 3 bind group layout"),
                entries: &[
                    // Vertex_buffer
                    crate::utils::compute_buffer_bind_group_layout_entry(0, false),
                ],
            });

        let pipeline_stage_1 = create_compute_pipeline(
            device,
            "surface sampler stage 1",
            include_str!("surface_sampler_stage_1.wgsl"),
            &bind_group_layout_stage_1,
            "main",
        );
        let pipeline_stage_2 = create_compute_pipeline(
            device,
            "surface sampler stage 2",
            include_str!("surface_sampler_stage_2.wgsl"),
            &bind_group_layout_stage_2,
            "main",
        );
        let pipeline_stage_3 = create_compute_pipeline(
            device,
            "surface sampler stage 3",
            include_str!("surface_sampler_stage_3.wgsl"),
            &bind_group_layout_stage_3,
            "main",
        );

        SurfaceSampler {
            renderer: renderer.clone(),
            bind_group_layout_stage_1,
            bind_group_layout_stage_2,
            bind_group_layout_stage_3,
            pipeline_stage_1,
            pipeline_stage_2,
            pipeline_stage_3,
            index_buffer_generator: IndexBufferGenerator::new(device),
        }
    }

    fn create_basis_funcs(
        &self,
        control_count_u: u32,
        control_count_v: u32,
        degree_u: u32,
        degree_v: u32,
        knots_u: &[f32],
        knots_v: &[f32],
        span_buffer_u: &wgpu::Buffer,
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
            control_count_v,
            degree_u,
            degree_v,
            knots_u,
            knots_v,
            &span_buffer_u,
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
        let bind_group_stage_3: wgpu::BindGroup =
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("surface sampler bind group"),
                layout: &self.bind_group_layout_stage_3,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: samples.as_entire_binding(),
                }],
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

        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("surface sampler stage 2 compute pass"),
                timestamp_writes: None,
            });
            compute_pass.set_pipeline(&self.pipeline_stage_3);
            compute_pass.set_bind_group(0, &bind_group_stage_3, &[]);
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
    pub fn get_renderer(&self) -> Rc<Renderer> {
        self.renderer.clone()
    }
}
