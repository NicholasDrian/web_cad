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
        let bb_buffer_generator_bind_group_layout =
            renderer
                .get_device()
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("mesh bb buffer generator"),
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage { read_only: true },
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
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
