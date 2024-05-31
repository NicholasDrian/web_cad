// currently using a sub optimal algorithm.
// The depth is optimal but the work n log n.
// The optimal algorithm seems too complex to be worth the meager savings
// TODO: improve work complexity later
use std::rc::Rc;

use wgpu::util::DeviceExt;

use crate::{render::renderer::Renderer, utils::create_compute_pipeline};

use super::AlgorithmResources;

pub struct PrefixSumGenerator {
    renderer: Rc<Renderer>,
    prefix_sum_bind_group_layout: wgpu::BindGroupLayout,
    prefix_sum_pipeline: wgpu::ComputePipeline,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct PrefixSumUniform {
    offset: u32,
}

pub fn create_prefix_sum_resources(
    renderer: &Renderer,
) -> (wgpu::BindGroupLayout, wgpu::ComputePipeline) {
    let device = renderer.get_device();

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("prefix sum"),
        entries: &[
            // Params
            crate::utils::compute_uniform_bind_group_layout_entry(0),
            // Values
            crate::utils::compute_buffer_bind_group_layout_entry(1, true),
            // Next
            crate::utils::compute_buffer_bind_group_layout_entry(2, false),
        ],
    });

    let pipeline = create_compute_pipeline(
        device,
        "prefix sum",
        include_str!("prefix_sum.wgsl"),
        &bind_group_layout,
        "prefix_sum",
    );
    (bind_group_layout, pipeline)
}

pub fn prefix_sum(
    resources: &AlgorithmResources,
    values: &wgpu::Buffer,
    value_count: u32,
) -> (wgpu::Buffer, u32) {
    let device = resources.get_renderer().get_device();
    let (bind_group_layout, pipeline) = resources.get_resources(super::Algorithm::PrefixSum);

    let descriptor = &wgpu::BufferDescriptor {
        label: Some("prefix sum next buffer"),
        size: (value_count + 1) as u64 * std::mem::size_of::<f32>() as u64,
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    };
    // I think this is the best way to read sum from prefixu sum buffer
    let intermediate = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("prefix sum intermediate"),
        size: 4,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    // These buffers should be effectively zero initialized
    // This is very important
    let buffer_a = device.create_buffer(descriptor);
    let buffer_b = device.create_buffer(descriptor);

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("prefix sum"),
    });

    let iterations = f32::log2((value_count + 1) as f32).ceil() as u32;

    encoder.copy_buffer_to_buffer(values, 0, &buffer_a, 4, value_count as u64 * 4);

    for i in 0..iterations {
        let offset = u32::pow(2, i);

        let params = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("prefix sum params"),
            contents: bytemuck::cast_slice(&[PrefixSumUniform { offset }]),
            usage: wgpu::BufferUsages::UNIFORM,
        });

        let bind_group = if i & 1 == 0 {
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("prefix sum even"),
                layout: bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: params.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: buffer_a.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: buffer_b.as_entire_binding(),
                    },
                ],
            })
        } else {
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("prefix sum even"),
                layout: bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: params.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: buffer_b.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: buffer_a.as_entire_binding(),
                    },
                ],
            })
        };

        let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("prefix sum"),
            timestamp_writes: None,
        });
        compute_pass.set_pipeline(pipeline);
        compute_pass.set_bind_group(0, &bind_group, &[]);

        compute_pass.dispatch_workgroups(value_count - offset, 1, 1);
    }

    let res = if iterations & 1 == 0 {
        buffer_a
    } else {
        buffer_b
    };

    encoder.copy_buffer_to_buffer(&res, (value_count - 1) as u64 * 4, &intermediate, 0, 4);

    let idx = resources.renderer.get_queue().submit([encoder.finish()]);
    device.poll(wgpu::Maintain::WaitForSubmissionIndex(idx));

    // might be doing this wrong
    let sum_bytes: &[u8] = &intermediate.slice(..).get_mapped_range();

    // TODO: see if this should be big endian sometimes?
    let sum = u32::from_le_bytes(sum_bytes[0..4].try_into().unwrap());

    (res, sum)
}
