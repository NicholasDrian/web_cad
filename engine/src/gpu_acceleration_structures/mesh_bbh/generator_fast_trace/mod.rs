//! BBH generator optimized for fast tracing
//!
//! TODO: factor
//! TODO: less parallel for bigger input
//!
//! TODO: deal with degen
//! TODO: deal with duplicates
//! NOTE: could use random jitter to do this
//! Note degenerates will break this
//!
use std::rc::Rc;

use js_sys::Date;
use wgpu::util::DeviceExt;

use crate::{
    geometry::mesh::MeshVertex,
    gpu_acceleration_structures::mesh_bbh::NODE_SIZE,
    gpu_algorithms::{iota::iota, AlgorithmResources},
    profiling::stats::Stats,
    render::renderer::Renderer,
    utils::create_compute_pipeline,
};

use super::{MeshBBH, MAX_TRIS_PER_LEAF, SPLIT_CANDIDATES, SPLIT_EVALUATION_SIZE};

pub struct MeshBBHGeneratorFastTrace {
    renderer: Rc<Renderer>,
    algorithm_resources: Rc<AlgorithmResources>,

    create_triangle_bbs_bind_group_layout: wgpu::BindGroupLayout,
    create_triangle_bbs_pipeline: wgpu::ComputePipeline,

    find_node_offsets_bind_group_layout: wgpu::BindGroupLayout,
    find_node_offsets_pipeline: wgpu::ComputePipeline,

    build_bbs_bind_group_layout: wgpu::BindGroupLayout,
    build_bbs_pipeline: wgpu::ComputePipeline,

    split_evaluations_bind_group_layout: wgpu::BindGroupLayout,
    split_evaluations_pipeline: wgpu::ComputePipeline,

    build_next_level_bind_group_layout: wgpu::BindGroupLayout,
    build_next_level_pipeline: wgpu::ComputePipeline,

    stats: Stats,
}

impl MeshBBHGeneratorFastTrace {
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
        let find_node_offsets_bind_group_layout =
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
                    crate::utils::compute_buffer_bind_group_layout_entry(1, true),
                    // triangle bbs
                    crate::utils::compute_buffer_bind_group_layout_entry(2, true),
                    // Tree
                    crate::utils::compute_buffer_bind_group_layout_entry(3, false),
                ],
            });
        let split_evaluations_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("split evaluations"),
                entries: &[
                    // Params
                    crate::utils::compute_uniform_bind_group_layout_entry(0),
                    // Index Buffer
                    crate::utils::compute_buffer_bind_group_layout_entry(1, true),
                    // triangle bbs
                    crate::utils::compute_buffer_bind_group_layout_entry(2, true),
                    // Tree
                    crate::utils::compute_buffer_bind_group_layout_entry(3, true),
                    // Result
                    crate::utils::compute_buffer_bind_group_layout_entry(4, false),
                ],
            });
        let build_next_level_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("build next level"),
                entries: &[
                    // Params
                    crate::utils::compute_uniform_bind_group_layout_entry(0),
                    // triangle bbs
                    crate::utils::compute_buffer_bind_group_layout_entry(1, true),
                    // Split evals
                    crate::utils::compute_buffer_bind_group_layout_entry(2, true),
                    // Prefix sum
                    crate::utils::compute_buffer_bind_group_layout_entry(3, true),
                    // Index Buffer
                    crate::utils::compute_buffer_bind_group_layout_entry(4, false),
                    // Tree
                    crate::utils::compute_buffer_bind_group_layout_entry(5, false),
                ],
            });
        let create_triangle_bbs_pipeline = create_compute_pipeline(
            device,
            "create triangle bbs",
            include_str!("create_triangle_bbs.wgsl"),
            &create_triangle_bbs_bind_group_layout,
            "generate_bb_buffer",
        );
        let find_node_offsets_pipeline = create_compute_pipeline(
            device,
            "find node offsets",
            include_str!("find_node_offsets.wgsl"),
            &find_node_offsets_bind_group_layout,
            "main",
        );
        let build_bbs_pipeline = create_compute_pipeline(
            device,
            "build bbs",
            include_str!("build_bbs.wgsl"),
            &build_bbs_bind_group_layout,
            "build_bbs",
        );
        let split_evaluations_pipeline = create_compute_pipeline(
            device,
            "split evaluations",
            include_str!("split_evaluations.wgsl"),
            &split_evaluations_bind_group_layout,
            "main",
        );
        let build_next_level_pipeline = create_compute_pipeline(
            device,
            "build next level",
            include_str!("build_next_level.wgsl"),
            &build_next_level_bind_group_layout,
            "build_next_level",
        );

        Self {
            renderer,
            algorithm_resources,
            create_triangle_bbs_bind_group_layout,
            create_triangle_bbs_pipeline,
            find_node_offsets_bind_group_layout,
            find_node_offsets_pipeline,
            build_bbs_bind_group_layout,
            build_bbs_pipeline,
            split_evaluations_bind_group_layout,
            split_evaluations_pipeline,
            build_next_level_bind_group_layout,
            build_next_level_pipeline,
            stats: Stats::new(),
        }
    }
    pub async fn generate_mesh_bbh(
        &self,
        vertex_buffer: &wgpu::Buffer,
        vertex_count: u32,
        mesh_index_buffer: &wgpu::Buffer,
        mesh_index_count: u32,
    ) -> MeshBBH {
        let start_time = Date::now();
        let triangle_count = mesh_index_count / 3;
        let triangle_bbs: wgpu::Buffer = self.create_triangle_bbs(
            vertex_buffer,
            vertex_count,
            mesh_index_buffer,
            mesh_index_count,
        );
        let index_buffer = iota(&self.algorithm_resources, triangle_count, 16);
        let tree_buffer = self.init_tree_buffer(mesh_index_count);
        let mut input: (u32, u32) = (0, 1);
        let mut level = 0;
        loop {
            if level == 100 {
                // TODO: shouldnt need this
                break;
            }
            // TODO: remove this in favor of bottom up approach
            self.build_bbs(&tree_buffer, &index_buffer, &triangle_bbs, input);

            // prefix sum of number of nodes with children
            // TODO: use prefix sum for this
            let (prefix_sum, total) = self.find_node_offsets(&tree_buffer, input).await;
            if total == 0 {
                // Input is all leaves. were done
                break;
            }

            let split_evaluations =
                self.split_evaluations(&tree_buffer, &index_buffer, &triangle_bbs, input);

            self.build_next_level(
                &tree_buffer,
                &index_buffer,
                &triangle_bbs,
                &split_evaluations,
                &prefix_sum,
                input,
            );

            input = (input.1, input.1 + total * 2);
            level += 1;
        }

        // eliminate extra capacity
        // TODO: factor this out
        let final_tree_buffer = self
            .renderer
            .get_device()
            .create_buffer(&wgpu::BufferDescriptor {
                label: Some("final tree buffer"),
                size: (input.1 * NODE_SIZE) as u64,
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
                mapped_at_creation: false,
            });

        let mut encoder =
            self.renderer
                .get_device()
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("shrink tree buffer"),
                });

        encoder.copy_buffer_to_buffer(
            &tree_buffer,
            0,
            &final_tree_buffer,
            0,
            (input.1 * NODE_SIZE) as u64,
        );

        let idx = self.renderer.get_queue().submit([encoder.finish()]);

        self.renderer
            .get_device()
            .poll(wgpu::Maintain::WaitForSubmissionIndex(idx));

        self.stats
            .add("mesh bbh creation time", Date::now() - start_time);
        log::info!("{:}", self.stats);

        MeshBBH::new(final_tree_buffer, index_buffer, input.1)
    }

    // Good ez paralelism
    fn create_triangle_bbs(
        &self,
        vertex_buffer: &wgpu::Buffer,
        vertex_count: u32,
        index_buffer: &wgpu::Buffer,
        index_count: u32,
    ) -> wgpu::Buffer {
        let start_time = Date::now();
        let device = self.renderer.get_device();

        let triangle_count = index_count / 3;
        let triangle_info_size = 16 * 3;
        let bb_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("create bb buffer"),
            // Check this
            size: (triangle_count * triangle_info_size) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        let vertex_buffer_clone = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("vert buff"),
            size: vertex_count as u64 * std::mem::size_of::<MeshVertex>() as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
            mapped_at_creation: false,
        });
        let index_buffer_clone = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("index buff"),
            size: index_count as u64 * std::mem::size_of::<u32>() as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
            mapped_at_creation: false,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("create bb buffer"),
            layout: &self.create_triangle_bbs_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    // TODO: copy this to storage or add storage flag to mesh
                    resource: vertex_buffer_clone.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: index_buffer_clone.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: bb_buffer.as_entire_binding(),
                },
            ],
        });
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("create bb buffer"),
        });

        encoder.copy_buffer_to_buffer(
            index_buffer,
            0,
            &index_buffer_clone,
            0,
            index_count as u64 * std::mem::size_of::<u32>() as u64,
        );
        encoder.copy_buffer_to_buffer(
            vertex_buffer,
            0,
            &vertex_buffer_clone,
            0,
            vertex_count as u64 * std::mem::size_of::<MeshVertex>() as u64,
        );

        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("create bb buffer"),
                timestamp_writes: None,
            });

            compute_pass.set_pipeline(&self.create_triangle_bbs_pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);
            compute_pass.dispatch_workgroups(triangle_count, 1, 1);
        }

        let idx = self.renderer.get_queue().submit([encoder.finish()]);
        device.poll(wgpu::Maintain::WaitForSubmissionIndex(idx));

        self.stats
            .add("create triangle bbs", Date::now() - start_time);

        bb_buffer
    }

    // EZ
    fn init_tree_buffer(&self, index_count: u32) -> wgpu::Buffer {
        let triangle_count = index_count / 3;
        let device = self.renderer.get_device();
        let tree_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("tree buffer"),
            size: (NODE_SIZE * triangle_count * 2) as u64, // bigger than neccesary
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: true,
        });
        let start = 0;
        let end = triangle_count;

        {
            let mut buffer_view = tree_buffer.slice(..).get_mapped_range_mut();
            let data: &mut [u32] = bytemuck::cast_slice_mut(&mut buffer_view);
            data[7] = start;
            data[8] = end;
        }

        tree_buffer.unmap();

        tree_buffer
    }

    // Prefix sum over nodes in current level that need children. Allows building next level in parallel.
    // TODO:, make parallel lol;
    async fn find_node_offsets(
        &self,
        tree: &wgpu::Buffer,
        range: (u32, u32),
    ) -> (wgpu::Buffer, u32) {
        let start_time = Date::now();

        let device = self.renderer.get_device();
        let params = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("create prefix sum input"),
            contents: bytemuck::cast_slice(&[range.0, range.1, MAX_TRIS_PER_LEAF]),
            usage: wgpu::BufferUsages::UNIFORM,
        });
        // zero initialized
        let result = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("prefix sum"),
            size: (range.1 - range.0 + 1) as u64 * std::mem::size_of::<u32>() as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        let sum_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("sum"),
            size: 4,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("create prefix sum input"),
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("create prefix sum input"),
            layout: &self.find_node_offsets_bind_group_layout,
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
                    resource: result.as_entire_binding(),
                },
            ],
        });

        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("create prefix sum input"),
                timestamp_writes: None,
            });

            compute_pass.set_pipeline(&self.find_node_offsets_pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);
            compute_pass.dispatch_workgroups(1, 1, 1);
        }

        encoder.copy_buffer_to_buffer(&result, (range.1 - range.0) as u64 * 4, &sum_buffer, 0, 4);

        let idx = self.renderer.get_queue().submit([encoder.finish()]);
        device.poll(wgpu::Maintain::WaitForSubmissionIndex(idx));

        self.stats
            .add("find node children", Date::now() - start_time);

        let start_time = Date::now();

        // TODO: eliminate this
        // this read is from VRAM is huge bottleneck
        let (sender, receiver) = futures::channel::oneshot::channel();

        let sum = {
            let slice = sum_buffer.slice(..);
            slice.map_async(wgpu::MapMode::Read, |result| {
                let _ = sender.send(result);
            });

            receiver
                .await
                .expect("communication failed")
                .expect("buffer reading failed");

            let bytes: [u8; 4] = slice.get_mapped_range()[0..4].try_into().unwrap();
            u32::from_le_bytes(bytes)
        };

        self.stats
            .add("read sum from gpu", Date::now() - start_time);

        (result, sum)
    }

    // TODO: replace this with bottum up version
    fn build_bbs(
        &self,
        tree: &wgpu::Buffer,
        indices: &wgpu::Buffer,
        triangle_bbs: &wgpu::Buffer,
        range: (u32, u32),
    ) {
        let start_time = Date::now();
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
                    binding: 3,
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

        self.stats.add("build bbs", Date::now() - start_time);
    }

    // TODO: make more paralel
    fn split_evaluations(
        &self,
        tree_buffer: &wgpu::Buffer,
        index_buffer: &wgpu::Buffer,
        triangle_bbs: &wgpu::Buffer,
        range: (u32, u32),
    ) -> wgpu::Buffer {
        let start_time = Date::now();
        let device = self.renderer.get_device();
        let params = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("split evaluations"),
            contents: bytemuck::cast_slice(&[range.0, MAX_TRIS_PER_LEAF, SPLIT_CANDIDATES]),
            usage: wgpu::BufferUsages::UNIFORM,
        });
        let res = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("split evaluations"),
            size: ((range.1 - range.0) * SPLIT_EVALUATION_SIZE * SPLIT_CANDIDATES) as u64,
            usage: wgpu::BufferUsages::STORAGE,
            mapped_at_creation: false,
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("split evaluations"),
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("split evaluations"),
            layout: &self.split_evaluations_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: params.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: index_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: triangle_bbs.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: tree_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: res.as_entire_binding(),
                },
            ],
        });

        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("split evaluations"),
                timestamp_writes: None,
            });

            compute_pass.set_pipeline(&self.split_evaluations_pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);
            compute_pass.dispatch_workgroups(range.1 - range.0, SPLIT_CANDIDATES, 1);
        }

        let idx = self.renderer.get_queue().submit([encoder.finish()]);
        device.poll(wgpu::Maintain::WaitForSubmissionIndex(idx));

        self.stats
            .add("split evaluations", Date::now() - start_time);
        res
    }

    // reorder indices and write out next level
    // set child pointers
    fn build_next_level(
        &self,
        tree_buffer: &wgpu::Buffer,
        index_buffer: &wgpu::Buffer,
        triangle_bbs: &wgpu::Buffer,
        split_evaluations: &wgpu::Buffer,
        prefix_sum: &wgpu::Buffer,
        input: (u32, u32),
    ) {
        let start_time = Date::now();
        let device = self.renderer.get_device();
        let params = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("build next level"),
            contents: bytemuck::cast_slice(&[input.0, MAX_TRIS_PER_LEAF, SPLIT_CANDIDATES]),
            usage: wgpu::BufferUsages::UNIFORM,
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("build next level"),
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("build next level"),
            layout: &self.build_next_level_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: params.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: triangle_bbs.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: split_evaluations.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: prefix_sum.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: index_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 5,
                    resource: tree_buffer.as_entire_binding(),
                },
            ],
        });

        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("build next level"),
                timestamp_writes: None,
            });

            compute_pass.set_pipeline(&self.build_next_level_pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);
            compute_pass.dispatch_workgroups(input.1 - input.0, 1, 1);
        }

        let idx = self.renderer.get_queue().submit([encoder.finish()]);
        device.poll(wgpu::Maintain::WaitForSubmissionIndex(idx));

        self.stats.add("build next level", Date::now() - start_time);
    }
}
