//! WARN: Creating a bb for a mesh alters its index buffer.
//!
//! Mesh BBH creation uses the following steps
//!
//! 1.) generate bounding box for each triangle
//!
//! 2.) create a buffer of necessary splits
//!
//! 3.) use bfs to find next split buffer.
//!
//! 4.) repeat till everything is split.
//!
//! 5.) compact
//!
use std::rc::Rc;

use crate::geometry::mesh::Mesh;
use crate::math::linear_algebra::vec3::Vec3;
use crate::render::renderer::Renderer;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct BoundingBox {
    // Todo: verify alignment
    min: Vec3,
    max: Vec3,
    center: Vec3,
    area: f32,
}

pub struct MeshBBHGenerator {
    renderer: Rc<Renderer>,

    bb_buffer_generator_bind_group_layout: wgpu::BindGroupLayout,
    split_stage_1_bind_group_layout: wgpu::BindGroupLayout,
    split_stage_2_bind_group_layout: wgpu::BindGroupLayout,
    compact_bind_group_layout: wgpu::BindGroupLayout,
    calculate_bbs_bind_group_layout: wgpu::BindGroupLayout,

    bb_buffer_generator_pipeline: wgpu::ComputePipeline,
    // determine where to split
    split_stage_1_pipeline: wgpu::ComputePipeline,
    // perform split
    split_stage_2_pipeline: wgpu::ComputePipeline,
    complat_pipeline: wgpu::ComputePipeline,
    calculate_bbs_pipeline: wgpu::ComputePipeline,
}

impl MeshBBHGenerator {
    pub fn new(renderer: Rc<Renderer>) -> Self {
        let device = renderer.get_device();
        let create_mesh_bb_buffer_shader_module =
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("mesh bb buffer generator"),
                source: wgpu::ShaderSource::Wgsl(include_str!("create_mesh_bb_buffer.wgsl").into()),
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

    pub fn create_bbh(&self, mesh: &Mesh) -> wgpu::Buffer {
        todo!()
    }

    fn create_bb_buffer(mesh: &Mesh) -> wgpu::Buffer {
        todo!();
    }

    fn find_splits(mesh: &Mesh, segments: &wgpu::Buffer) -> wgpu::Buffer {
        todo!();
    }

    fn perform_splits(mesh: &Mesh, segments: &wgpu::Buffer) -> wgpu::Buffer {
        todo!();
    }

    fn compact(bbh: &wgpu::Buffer) {}
}
