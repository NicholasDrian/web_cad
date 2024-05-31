use std::rc::Rc;

use wgpu::util::DeviceExt;

use crate::{
    geometry::mesh::Mesh,
    gpu_algorithms::{iota::iota, prefix_sum::prefix_sum, AlgorithmResources},
    render::renderer::Renderer,
    utils::create_compute_pipeline,
};

use super::mesh_bbh::MeshBBH;

const NODE_SIZE: u32 = 48;
const MAX_TRIS_PER_LEAF: u32 = 8;

pub struct MeshBBHGenerator {
    renderer: Rc<Renderer>,
    algorithm_resources: Rc<AlgorithmResources>,

    create_triangle_bbs_bind_group_layout: wgpu::BindGroupLayout,
    create_triangle_bbs_pipeline: wgpu::ComputePipeline,

    create_prefix_sum_input_bind_group_layout: wgpu::BindGroupLayout,
    create_prefix_sum_input_pipeline: wgpu::ComputePipeline,

    build_bbs_bind_group_layout: wgpu::BindGroupLayout,
    build_bbs_pipeline: wgpu::ComputePipeline,
}

impl MeshBBHGenerator {
    pub fn new(renderer: Rc<Renderer>, algorithm_resources: Rc<AlgorithmResources>) -> Self {
        let device = renderer.get_device();
        let create_triangle_bbs_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("create triangle bbs"),
                entries: &[
                    // Vertex
                    crate::utils::compute_buffer_bind_group_layout_entry(0, true),
                    // Index
                    crate::utils::compute_buffer_bind_group_layout_entry(1, true),
                    // bb_buffer
                    crate::utils::compute_buffer_bind_group_layout_entry(2, false),
                ],
            });
        let create_prefix_sum_input_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("create prefix sum input"),
                entries: &[
                    // Params
                    crate::utils::compute_uniform_bind_group_layout_entry(0),
                    // Tree
                    crate::utils::compute_buffer_bind_group_layout_entry(1, true),
                    // Output
                    crate::utils::compute_buffer_bind_group_layout_entry(2, false),
                ],
            });
        let build_bbs_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("build bbs"),
                entries: &[
                    // Params
                    crate::utils::compute_uniform_bind_group_layout_entry(0),
                    // Index Buffer
                    crate::utils::compute_buffer_bind_group_layout_entry(2, true),
                    // triangle bbs
                    crate::utils::compute_buffer_bind_group_layout_entry(3, true),
                    // Tree
                    crate::utils::compute_buffer_bind_group_layout_entry(1, false),
                ],
            });
        let create_triangle_bbs_pipeline = create_compute_pipeline(
            device,
            "create triangle bbs",
            include_str!("create_triangle_bbs.wgsl"),
            &create_triangle_bbs_bind_group_layout,
            "generate_bb_buffer",
        );
        let create_prefix_sum_input_pipeline = create_compute_pipeline(
            device,
            "create prefix sum input",
            include_str!("create_prefix_sum_input.wgsl"),
            &create_prefix_sum_input_bind_group_layout,
            "main",
        );
        let build_bbs_pipeline = create_compute_pipeline(
            device,
            "build bbs",
            include_str!("create_prefix_sum_input.wgsl"),
            &build_bbs_bind_group_layout,
            "build_bbs",
        );
        Self {
            renderer,
            algorithm_resources,
            create_triangle_bbs_bind_group_layout,
            create_triangle_bbs_pipeline,
            create_prefix_sum_input_bind_group_layout,
            create_prefix_sum_input_pipeline,
            build_bbs_bind_group_layout,
            build_bbs_pipeline,
        }
    }
    pub fn generate_mesh_bbh(&self, mesh: &Mesh) -> MeshBBH {
        let triangle_count = mesh.get_index_count() / 3;
        let triangle_bbs: wgpu::Buffer = self.create_triangle_bbs(mesh);
        let index_buffer = iota(&self.algorithm_resources, triangle_count, 16);
        let tree_buffer = self.init_tree_buffer(mesh);
        let mut input: (u32, u32) = (0, 1);
        loop {
            // TODO: remove this in favor of bottom up approach
            self.build_bbs(&tree_buffer, &index_buffer, &triangle_bbs, input);

            // prefix sum of number of nodes with children
            let (prefix_sum, total) = self.prefix_sum(&tree_buffer, input);
            if (total == 0) {
                // Input is all leaves. were done
                break;
            }

            let split_evaluations =
                self.split_evaluations(&tree_buffer, &index_buffer, &triangle_bbs, input);

            self.build_next_level(
                &tree_buffer,
                &index_buffer,
                &split_evaluations,
                &prefix_sum,
                input,
            );

            input = (input.1, input.1 + total);
        }

        // TODO: shrink tree buffer to size of tree

        MeshBBH::new(tree_buffer, index_buffer)
    }

    // Good ez paralelism
    fn create_triangle_bbs(&self, mesh: &Mesh) -> wgpu::Buffer {
        let device = self.renderer.get_device();

        let triangle_count = mesh.get_index_count() / 3;
        let triangle_info_size = 16 * 3;
        let bb_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("create bb buffer"),
            // Check this
            size: (triangle_count * triangle_info_size) as u64,
            usage: wgpu::BufferUsages::STORAGE,
            mapped_at_creation: false,
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("create bb buffer"),
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("create bb buffer"),
            layout: &self.create_triangle_bbs_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: mesh.get_vertex_buffer().as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: mesh.get_index_buffer().as_entire_binding(),
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

            compute_pass.set_pipeline(&self.create_triangle_bbs_pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);
            compute_pass.dispatch_workgroups(triangle_count / 3, 1, 1);
        }

        let idx = self.renderer.get_queue().submit([encoder.finish()]);
        device.poll(wgpu::Maintain::WaitForSubmissionIndex(idx));

        bb_buffer
    }

    // EZ
    fn init_tree_buffer(&self, mesh: &Mesh) -> wgpu::Buffer {
        let triangle_count = mesh.get_index_count() / 3;
        let device = self.renderer.get_device();
        let tree_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("tree buffer"),
            size: (NODE_SIZE * triangle_count * 2) as u64, // bigger than neccesary
            usage: wgpu::BufferUsages::STORAGE,
            mapped_at_creation: true,
        });
        let queue = self.renderer.get_queue();
        let start = 0;
        let end = triangle_count;
        queue.write_buffer(
            &tree_buffer,
            0,
            bytemuck::cast_slice(&[0u32, 0, 0, start, 0, 0, 0, end]),
        );
        tree_buffer.unmap();
        tree_buffer
    }

    // I got this paralell
    fn prefix_sum(&self, tree: &wgpu::Buffer, range: (u32, u32)) -> (wgpu::Buffer, u32) {
        let device = self.renderer.get_device();
        let params = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("create prefix sum input"),
            contents: bytemuck::cast_slice(&[range.0, MAX_TRIS_PER_LEAF]),
            usage: wgpu::BufferUsages::UNIFORM,
        });
        let prefix_sum_input = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("prefix sum input"),
            size: ((range.1 - range.0) * NODE_SIZE) as u64,
            usage: wgpu::BufferUsages::STORAGE,
            mapped_at_creation: false,
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("create prefix sum input"),
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("create prefix sum input"),
            layout: &self.create_prefix_sum_input_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: params.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: tree.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: prefix_sum_input.as_entire_binding(),
                },
            ],
        });

        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("create prefix sum input"),
                timestamp_writes: None,
            });

            compute_pass.set_pipeline(&self.create_prefix_sum_input_pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);
            compute_pass.dispatch_workgroups(range.1 - range.0, 1, 1);
        }

        let idx = self.renderer.get_queue().submit([encoder.finish()]);
        device.poll(wgpu::Maintain::WaitForSubmissionIndex(idx));

        prefix_sum(
            &self.algorithm_resources,
            &prefix_sum_input,
            range.1 - range.0,
        )
    }

    // TODO: replace this with bottum up version
    fn build_bbs(
        &self,
        tree: &wgpu::Buffer,
        indices: &wgpu::Buffer,
        triangle_bbs: &wgpu::Buffer,
        range: (u32, u32),
    ) {
        let device = self.renderer.get_device();
        let params = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("build bbs"),
            contents: bytemuck::cast_slice(&[range.0]),
            usage: wgpu::BufferUsages::UNIFORM,
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("build bbs"),
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("build bbs"),
            layout: &self.build_bbs_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: params.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: indices.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: triangle_bbs.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: tree.as_entire_binding(),
                },
            ],
        });

        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("build bbs"),
                timestamp_writes: None,
            });

            compute_pass.set_pipeline(&self.build_bbs_pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);
            compute_pass.dispatch_workgroups(range.1 - range.0, 1, 1);
        }

        let idx = self.renderer.get_queue().submit([encoder.finish()]);
        device.poll(wgpu::Maintain::WaitForSubmissionIndex(idx));
    }

    // TODO: make more paralel
    fn split_evaluations(
        &self,
        tree_buffer: &wgpu::Buffer,
        index_buffer: &wgpu::Buffer,
        triangle_bbs: &wgpu::Buffer,
        input: (u32, u32),
    ) -> wgpu::Buffer {
        todo!()
    }

    // reorder indices and write out next level
    // set child pointers
    fn build_next_level(
        &self,
        tree_buffer: &wgpu::Buffer,
        index_buffer: &wgpu::Buffer,
        split_evaluations: &wgpu::Buffer,
        prefix_sum: &wgpu::Buffer,
        input: (u32, u32),
    ) {
        todo!()
    }
}
