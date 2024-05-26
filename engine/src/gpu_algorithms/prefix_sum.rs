// currently using a sub optimal algorithm.
// The depth is optimal but the work n log n.
// The optimal algorithm seems too complex to be worth the meager savings
use std::rc::Rc;

use crate::render::renderer::Renderer;

// TODO: improve using https://developer.nvidia.com/gpugems/gpugems3/part-vi-gpu-computing/chapter-39-parallel-prefix-sum-scan-cuda
pub fn get_prefix_sum_naieve(
    segments: &wgpu::Buffer,
    segment_count: u32,
    values: &wgpu::Buffer,
    value_count: u32,
) -> wgpu::Buffer {
    todo!()
}

pub struct PrefixSumGenerator {
    renderer: Rc<Renderer>,
    prefix_sum_bind_group_layout: wgpu::BindGroupLayout,
    segmentation_bind_group_layout: wgpu::BindGroupLayout,
    prefix_sum_pipeline: wgpu::ComputePipeline,
    segmentation_pipeline: wgpu::ComputePipeline,
}

impl PrefixSumGenerator {
    pub fn new(renderer: Rc<Renderer>) -> Self {
        let device = renderer.get_device();
        let prefix_sum_shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("prefix sum"),
            source: wgpu::ShaderSource::Wgsl(include_str!("prefix_sum.wgsl").into()),
        });

        let bb_buffer_generator_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("mesh bb buffer generator"),
                entries: &[
                    // Vertex
                    crate::utils::compute_buffer_bind_group_layout_entry(0, true),
                    // Index
                    crate::utils::compute_buffer_bind_group_layout_entry(1, true),
                    // bb_buffer
                    crate::utils::compute_buffer_bind_group_layout_entry(2, false),
                ],
            });

        todo!()
    }
}
