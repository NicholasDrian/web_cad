//! create buffer with sequential numbers on the GPU

use std::rc::Rc;

use crate::render::renderer::Renderer;

pub fn create_iota_resources(
    renderer: &Renderer,
) -> (wgpu::BindGroupLayout, wgpu::ComputePipeline) {
    todo!()
}

/// Buffer will probably be a bit longer than requested length
pub fn iota(device: &wgpu::Device, length: u32) -> wgpu::Buffer {
    todo!()
}
