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

use wgpu::util::DeviceExt;

use crate::geometry::mesh::Mesh;
use crate::gpu_algorithms::{iota::iota, AlgorithmResources};
use crate::render::renderer::Renderer;
use crate::utils::create_compute_pipeline;

const SPLIT_CANDIDATE_COUNT: u32 = 8;
const MAX_LEAF_SIZE: u32 = 8;

pub struct MeshBBHGenerator {
    renderer: Rc<Renderer>,
    algorithm_resources: Rc<AlgorithmResources>,

    triangle_info_buffer_generator_bind_group_layout: wgpu::BindGroupLayout,
    triangle_info_buffer_generator_pipeline: wgpu::ComputePipeline,

    split_bind_group_layout: wgpu::BindGroupLayout,
    split_pipeline: wgpu::ComputePipeline,
}

impl MeshBBHGenerator {
    pub fn new(renderer: Rc<Renderer>, algorithm_resources: Rc<AlgorithmResources>) -> Self {
        let device = renderer.get_device();

        let triangle_info_buffer_generator_bind_group_layout =
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
        let split_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("split finder"),
                entries: &[
                    // split finder params
                    crate::utils::compute_uniform_bind_group_layout_entry(0),
                    // splits
                    crate::utils::compute_buffer_bind_group_layout_entry(1, true),
                    // new_splits
                    crate::utils::compute_buffer_bind_group_layout_entry(2, false),
                ],
            });

        let triangle_info_buffer_generator_pipeline = create_compute_pipeline(
            device,
            "bb buffer gen",
            include_str!("create_triangle_info_buffer.wgsl"),
            &triangle_info_buffer_generator_bind_group_layout,
            "generate_bb_buffer",
        );
        let split_pipeline = create_compute_pipeline(
            device,
            "split finder",
            include_str!("create_mesh_bbh_split.wgsl"),
            &split_bind_group_layout,
            "find_splits",
        );

        Self {
            renderer,
            algorithm_resources,
            triangle_info_buffer_generator_bind_group_layout,
            triangle_info_buffer_generator_pipeline,
            split_bind_group_layout,
            split_pipeline,
        }
    }

    pub fn create_bbh(&self, mesh: &Mesh) -> wgpu::Buffer {
        let triangle_info_buffer = self.create_triangle_info_buffer(
            mesh.get_vertex_buffer(),
            mesh.get_index_buffer(),
            mesh.get_index_count(),
        );

        // one index per triangle.
        let bbh_index_buffer = iota(&self.algorithm_resources, mesh.get_index_count() / 3, 16);

        // create levels and init with first split queue.
        type LevelLength = u32;
        let mut levels: Vec<(wgpu::Buffer, LevelLength)> = vec![(
            self.renderer
                .get_device()
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("initial split"),
                    contents: bytemuck::cast_slice(&[0u32, mesh.get_index_count() / 3]),
                    usage: wgpu::BufferUsages::STORAGE,
                }),
            1u32,
        )];

        loop {
            //  let splits = self.split(&triangle_info_buffer, levels.last().unwrap());

            /*
                        // make sure final sum is left and right. differnt than prefix_sum.back()
                        let (prefix_sum, final_sum) = self.prefix_sum_generator(possible_splits);

                        if final_sum == 0 {
                            break;
                        }

                        // reorder index_buffer

                        levels.push(self.creat_next_level(possible_splits, prefix_sum));
            */
        }

        todo!()
        // bottom up bb construction
        //return self.create_bbs(levels);
    }

    fn create_triangle_info_buffer(
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
            layout: &self.triangle_info_buffer_generator_bind_group_layout,
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

            compute_pass.set_pipeline(&self.triangle_info_buffer_generator_pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);
            compute_pass.dispatch_workgroups(index_count / 3, 1, 1);
        }

        let idx = self.renderer.get_queue().submit([encoder.finish()]);
        device.poll(wgpu::Maintain::WaitForSubmissionIndex(idx));

        bb_buffer
    }

    fn split(&self, triangle_info_buffer: &wgpu::Buffer, segments: &wgpu::Buffer) -> wgpu::Buffer {
        todo!();
    }
}
