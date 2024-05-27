//! create buffer with sequential numbers on the GPU

use wgpu::util::DeviceExt;

use crate::{render::renderer::Renderer, utils::create_compute_pipeline};

use super::AlgorithmResources;

pub(crate) fn create_iota_resources(
    renderer: &Renderer,
) -> (wgpu::BindGroupLayout, wgpu::ComputePipeline) {
    let device = renderer.get_device();

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("prefix sum"),
        entries: &[
            // Params
            crate::utils::compute_uniform_bind_group_layout_entry(0),
            // Result
            crate::utils::compute_buffer_bind_group_layout_entry(1, false),
        ],
    });

    let pipeline = create_compute_pipeline(
        device,
        "iota",
        include_str!("iota.wgsl"),
        &bind_group_layout,
        "iota",
    );
    (bind_group_layout, pipeline)
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct IotaUniform {
    resolution: u32,
}

/// Buffer will probably be a bit longer than requested length
/// Buffer size will be increased to a multipe of resolution
/// A resolution of 16 is recommended
pub(crate) fn iota(resources: &AlgorithmResources, length: u32, resolution: u32) -> wgpu::Buffer {
    // round length up to nearest multiple of resolution
    let length = (length + resolution - 1) / resolution * resolution;
    let device = resources.get_renderer().get_device();
    let res = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("iota"),
        size: (length * 4) as u64,
        usage: wgpu::BufferUsages::STORAGE,
        mapped_at_creation: false,
    });

    let (bind_group_layout, pipeline) = resources.get_resources(super::Algorithm::Iota);

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("iota"),
    });

    let params = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("iota"),
        contents: bytemuck::cast_slice(&[IotaUniform { resolution }]),
        usage: wgpu::BufferUsages::UNIFORM,
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("iota"),
        layout: bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: params.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: res.as_entire_binding(),
            },
        ],
    });

    {
        let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("iota"),
            timestamp_writes: None,
        });
        compute_pass.set_pipeline(pipeline);
        compute_pass.set_bind_group(0, &bind_group, &[]);
        compute_pass.dispatch_workgroups(length / resolution, 1, 1);
    }

    let idx = resources.renderer.get_queue().submit([encoder.finish()]);
    device.poll(wgpu::Maintain::WaitForSubmissionIndex(idx));

    res
}
