use std::rc::Rc;

use wgpu::util::DeviceExt;

use crate::{
    gpu_samplers::params::SAMPLES_PER_SEGMENT, math::linear_algebra::vec4::Vec4,
    render::renderer::Renderer, utils::create_compute_pipeline,
};

use super::utils::create_span_buffer;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CurveSamplerUniforms {
    control_count: u32,
    knot_count: u32,
    degree: u32,
}

pub struct CurveSampler {
    renderer: Rc<Renderer>,
    bind_group_layout: wgpu::BindGroupLayout,
    pipeline: wgpu::ComputePipeline,
}

impl CurveSampler {
    pub fn new(renderer: Rc<Renderer>) -> CurveSampler {
        let device = renderer.get_device();

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("curve sampler bind group layout"),
            entries: &[
                // Params
                crate::utils::compute_uniform_bind_group_layout_entry(0),
                // Controls
                crate::utils::compute_buffer_bind_group_layout_entry(1, true),
                // Knots
                crate::utils::compute_buffer_bind_group_layout_entry(2, true),
                // Spans
                crate::utils::compute_buffer_bind_group_layout_entry(3, true),
                // Basis Funcs
                crate::utils::compute_buffer_bind_group_layout_entry(4, false),
                // Samples
                crate::utils::compute_buffer_bind_group_layout_entry(5, false),
            ],
        });

        let pipeline = create_compute_pipeline(
            device,
            "curve sampler pipeline",
            include_str!("curve_sampler.wgsl"),
            &bind_group_layout,
            "main",
        );

        CurveSampler {
            renderer,
            bind_group_layout,
            pipeline,
        }
    }

    pub fn sample_curve(
        &self,
        degree: u32,
        weighted_controls: &[Vec4],
        knots: &[f32],
    ) -> wgpu::Buffer {
        let device = self.renderer.get_device();
        let queue = self.renderer.get_queue();

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("curve sampler uniform buffer"),
            contents: bytemuck::cast_slice(&[CurveSamplerUniforms {
                control_count: weighted_controls.len() as u32,
                knot_count: knots.len() as u32,
                degree,
            }]),
            usage: wgpu::BufferUsages::UNIFORM,
        });

        let sample_count: u64 =
            SAMPLES_PER_SEGMENT as u64 * (weighted_controls.len() as u64 - 1) + 1;

        let samples: wgpu::Buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("curve sampler output sample buffer"),
            size: sample_count * 16,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        let output: wgpu::Buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("curve sampler output buffer"),
            size: sample_count * 16,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let basis_funcs: wgpu::Buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("curve sampler basis funcs buffer"),
            size: sample_count * (degree as u64 + 1) * 4,
            usage: wgpu::BufferUsages::STORAGE,
            mapped_at_creation: false,
        });

        let control_point_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("curve sample control point buffer"),
            contents: bytemuck::cast_slice(weighted_controls),
            usage: wgpu::BufferUsages::STORAGE,
        });

        let knot_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("curve sample knot buffer"),
            contents: bytemuck::cast_slice(knots),
            usage: wgpu::BufferUsages::STORAGE,
        });

        let span_buffer = create_span_buffer(device, knots, degree, sample_count as u32);

        let bind_group: wgpu::BindGroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("curve sampler bind group"),
            layout: &self.bind_group_layout,
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
                    resource: knot_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: span_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: basis_funcs.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 5,
                    resource: samples.as_entire_binding(),
                },
            ],
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
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

        let idx = queue.submit([encoder.finish()]);

        device.poll(wgpu::Maintain::WaitForSubmissionIndex(idx));

        output
    }

    pub fn get_renderer(&self) -> Rc<Renderer> {
        self.renderer.clone()
    }
}
