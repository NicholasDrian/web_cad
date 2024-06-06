//! data independent sort for ez parallelization
//! comparisons: n log n
//! depth tight bound: log n * log n

// iteration  sort_size  step_size
// 0          2          1
// 1          4          2
// 2          4          1
// 3          8          4
// 4          8          2
// 5          8          1

use wgpu::util::DeviceExt;

use crate::{render::renderer::Renderer, utils::create_compute_pipeline};

use super::AlgorithmResources;

pub fn create_bitonic_merge_sort_resources(
    renderer: &Renderer,
) -> (wgpu::BindGroupLayout, wgpu::ComputePipeline) {
    let device = renderer.get_device();

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("bitonic merge sort"),
        entries: &[
            // Params
            crate::utils::compute_uniform_bind_group_layout_entry(0),
            // Keys
            crate::utils::compute_buffer_bind_group_layout_entry(1, false),
            // Values
            crate::utils::compute_buffer_bind_group_layout_entry(2, false),
        ],
    });

    let pipeline = create_compute_pipeline(
        device,
        "bitonic merges sort",
        include_str!("bitonic_merge_sort.wgsl"),
        &bind_group_layout,
        "main",
    );
    (bind_group_layout, pipeline)
}

// for now, keys are i64 and values are u32
pub fn radix_sort(
    resources: &AlgorithmResources,
    keys: &wgpu::Buffer,
    values: &wgpu::Buffer,
    count: u32,
) {
    let device = resources.get_renderer().get_device();
    let queue = resources.get_renderer().get_queue();
    let num_threads = count / 2;

    let (bind_group_layout, pipeline) = resources.get_resources(super::Algorithm::BitonicMergeSort);

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("bitonic merge sort"),
    });

    let mut sort_size = 2u32;
    let mut step_size = 1u32;

    while sort_size <= count {
        let params = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("bitonic merge sort"),
            contents: bytemuck::cast_slice(&[sort_size, step_size, count]),
            usage: wgpu::BufferUsages::UNIFORM,
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("bitonic merge sort"),
            layout: bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: params.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: keys.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: values.as_entire_binding(),
                },
            ],
        });

        let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("bitonic merge sort"),
            timestamp_writes: None,
        });
        compute_pass.set_pipeline(pipeline);
        compute_pass.set_bind_group(0, &bind_group, &[]);
        compute_pass.dispatch_workgroups(num_threads, 1, 1);

        if step_size == 1 {
            step_size = sort_size;
            sort_size *= 2;
        } else {
            step_size /= 2;
        }
    }

    let idx = queue.submit([encoder.finish()]);
    device.poll(wgpu::Maintain::WaitForSubmissionIndex(idx));
}
