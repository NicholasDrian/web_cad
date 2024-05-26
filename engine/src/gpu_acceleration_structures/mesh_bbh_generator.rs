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
    split_finder_bind_group_layout: wgpu::BindGroupLayout,
    //split_bind_group_layout: wgpu::BindGroupLayout,
    //compact_bind_group_layout: wgpu::BindGroupLayout,
    //calculate_bbs_bind_group_layout: wgpu::BindGroupLayout,
    bb_buffer_generator_pipeline: wgpu::ComputePipeline,
    // determine where to split
    split_finder_pipeline: wgpu::ComputePipeline,
    // perform split
    //split_pipeline: wgpu::ComputePipeline,
    //complat_pipeline: wgpu::ComputePipeline,
    //calculate_bbs_pipeline: wgpu::ComputePipeline,
}

impl MeshBBHGenerator {
    pub fn new(renderer: Rc<Renderer>) -> Self {
        let device = renderer.get_device();

        let bb_buffer_generator_shader_module =
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("bb buffer gen"),
                source: wgpu::ShaderSource::Wgsl(include_str!("create_mesh_bb_buffer.wgsl").into()),
            });
        let split_finder_shader_module =
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("bb buffer gen"),
                source: wgpu::ShaderSource::Wgsl(include_str!("create_mesh_bb_buffer.wgsl").into()),
            });

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
        let split_finder_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("split finder"),
                entries: &[
                // TODO:

                ],
            });

        let bb_buffer_generator_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("bb buffer gen"),
                bind_group_layouts: &[&bb_buffer_generator_bind_group_layout],
                push_constant_ranges: &[],
            });
        let split_finder_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("split finder"),
                bind_group_layouts: &[&split_finder_bind_group_layout],
                push_constant_ranges: &[],
            });

        let bb_buffer_generator_pipeline =
            device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some("bb buffer gen"),
                layout: Some(&bb_buffer_generator_pipeline_layout),
                module: &bb_buffer_generator_shader_module,
                entry_point: "generate_bb_buffer",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            });
        let split_finder_pipeline =
            device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some("split finder"),
                layout: Some(&split_finder_pipeline_layout),
                module: &split_finder_shader_module,
                entry_point: "find_splits",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            });
        Self {
            renderer,
            bb_buffer_generator_bind_group_layout,
            bb_buffer_generator_pipeline,
            split_finder_bind_group_layout,
            split_finder_pipeline,
        }
    }

    pub fn create_bbh(&self, mesh: &Mesh) -> wgpu::Buffer {
        let bb_buffer = self.create_bb_buffer(
            mesh.get_vertex_buffer(),
            mesh.get_index_buffer(),
            mesh.get_index_count(),
        );

        todo!()
    }

    fn create_bb_buffer(
        &self,
        vertex_buffer: &wgpu::Buffer,
        index_buffer: &wgpu::Buffer,
        index_count: u32,
    ) -> wgpu::Buffer {
        let device = self.renderer.get_device();

        let bb_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("create bb buffer"),
            size: index_count as u64 * std::mem::size_of::<f32>() as u64,
            usage: wgpu::BufferUsages::STORAGE,
            mapped_at_creation: false,
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("create bb buffer"),
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("create bb buffer"),
            layout: &self.bb_buffer_generator_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: vertex_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: index_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: bb_buffer.as_entire_binding(),
                },
            ],
        });

        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("create bb buffer"),
                timestamp_writes: None,
            });

            compute_pass.set_pipeline(&self.bb_buffer_generator_pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);
            compute_pass.dispatch_workgroups(index_count / 3, 1, 1);
        }

        let idx = self.renderer.get_queue().submit([encoder.finish()]);
        device.poll(wgpu::Maintain::WaitForSubmissionIndex(idx));

        bb_buffer
    }

    fn find_splits(&self, mesh: &Mesh, segments: &wgpu::Buffer) -> wgpu::Buffer {
        todo!();
    }

    fn perform_splits(&self, mesh: &Mesh, segments: &wgpu::Buffer) -> wgpu::Buffer {
        todo!();
    }

    fn compact(&self, bbh: &wgpu::Buffer) {}
}
