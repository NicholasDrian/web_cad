//! BBH generator optimized for fast creation
//!
//! Sorts primitives along morton curve then partitions into tree
//!
//! The tree is level, this means that children are in predictable positions
//! Child pointer not required for this, but included to be generic
//!

use wgpu::util::DeviceExt;

use super::{MeshBBH, MeshBBHNode};
use crate::{
    geometry::mesh::MeshVertex,
    gpu_acceleration_structures::mesh_bbh::generator_fast_trace::{MAX_TRIS_PER_LEAF, NODE_SIZE},
    gpu_algorithms::{bitonic_merge_sort::bitonic_merge_sort, iota::iota},
    utils::{create_compute_pipeline, dump_buffer},
};

pub struct MeshBBHGeneratorFastBuild {
    renderer: std::rc::Rc<crate::render::renderer::Renderer>,
    algorithm_resources: std::rc::Rc<crate::gpu_algorithms::AlgorithmResources>,

    create_bbs_bind_group_layout: wgpu::BindGroupLayout,
    create_bbs_pipeline: wgpu::ComputePipeline,

    accumulate_bbs_bind_group_layout: wgpu::BindGroupLayout,
    accumulate_bbs_pipeline: wgpu::ComputePipeline,

    calculate_morton_codes_bind_group_layout: wgpu::BindGroupLayout,
    calculate_morton_codes_pipeline: wgpu::ComputePipeline,

    init_tree_bind_group_layout: wgpu::BindGroupLayout,
    init_tree_pipeline: wgpu::ComputePipeline,

    build_tree_bind_group_layout: wgpu::BindGroupLayout,
    build_tree_pipeline: wgpu::ComputePipeline,
}

impl MeshBBHGeneratorFastBuild {
    pub fn new(
        renderer: std::rc::Rc<crate::render::renderer::Renderer>,
        algorithm_resources: std::rc::Rc<crate::gpu_algorithms::AlgorithmResources>,
    ) -> Self {
        let device = renderer.get_device();
        let create_bbs_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("create bbs"),
                entries: &[
                    // Vertex_buffer
                    crate::utils::compute_buffer_bind_group_layout_entry(0, true),
                    // Index buffer
                    crate::utils::compute_buffer_bind_group_layout_entry(1, true),
                    // Triangle info buffer
                    crate::utils::compute_buffer_bind_group_layout_entry(2, false),
                ],
            });
        let accumulate_bbs_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("accumulate bbs"),
                entries: &[
                    // params
                    crate::utils::compute_uniform_bind_group_layout_entry(0),
                    // triangle_info_buffer copy
                    crate::utils::compute_buffer_bind_group_layout_entry(1, false),
                ],
            });
        let calculate_morton_codes_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("calculate morton codes bbs"),
                entries: &[
                    // bb_buffer
                    crate::utils::compute_buffer_bind_group_layout_entry(0, true),
                    // accumulated_bb
                    crate::utils::compute_buffer_bind_group_layout_entry(1, true),
                    // morton code buffer
                    crate::utils::compute_buffer_bind_group_layout_entry(2, false),
                ],
            });
        let init_tree_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("init tree"),
                entries: &[
                    // params
                    crate::utils::compute_uniform_bind_group_layout_entry(0),
                    // Triangle bbs
                    crate::utils::compute_buffer_bind_group_layout_entry(1, true),
                    // bbh index buffer
                    crate::utils::compute_buffer_bind_group_layout_entry(2, true),
                    // tree buffer
                    crate::utils::compute_buffer_bind_group_layout_entry(3, false),
                ],
            });
        let build_tree_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("build tree"),
                entries: &[
                    // params
                    crate::utils::compute_uniform_bind_group_layout_entry(0),
                    // tree buffer
                    crate::utils::compute_buffer_bind_group_layout_entry(1, false),
                ],
            });
        let create_bbs_pipeline = create_compute_pipeline(
            device,
            "create bbs",
            include_str!("create_bbs.wgsl"),
            &create_bbs_bind_group_layout,
            "main",
        );
        let accumulate_bbs_pipeline = create_compute_pipeline(
            device,
            "accumulate bbs",
            include_str!("accumulate_bbs.wgsl"),
            &accumulate_bbs_bind_group_layout,
            "main",
        );
        let calculate_morton_codes_pipeline = create_compute_pipeline(
            device,
            "calculate morton codes",
            include_str!("calculate_morton_codes.wgsl"),
            &calculate_morton_codes_bind_group_layout,
            "main",
        );
        let init_tree_pipeline = create_compute_pipeline(
            device,
            "init tree",
            include_str!("build_leaves.wgsl"),
            &init_tree_bind_group_layout,
            "main",
        );
        let build_tree_pipeline = create_compute_pipeline(
            device,
            "build tree",
            include_str!("build_tree.wgsl"),
            &build_tree_bind_group_layout,
            "main",
        );
        Self {
            renderer,
            algorithm_resources,
            create_bbs_bind_group_layout,
            create_bbs_pipeline,
            accumulate_bbs_bind_group_layout,
            accumulate_bbs_pipeline,
            calculate_morton_codes_bind_group_layout,
            calculate_morton_codes_pipeline,
            init_tree_bind_group_layout,
            init_tree_pipeline,
            build_tree_bind_group_layout,
            build_tree_pipeline,
        }
    }

    pub fn generate_mesh_bbh(
        &self,
        vertex_buffer: &wgpu::Buffer,
        vertex_count: u32,
        index_buffer: &wgpu::Buffer,
        index_count: u32,
    ) -> MeshBBH {
        let triangle_count = index_count / 3;
        let triangle_bbs =
            self.calculate_triangle_bbs(vertex_buffer, vertex_count, index_buffer, index_count);
        let accumulated_bb = self.accumulate_bbs(&triangle_bbs, triangle_count);
        let morton_codes =
            self.calculate_morton_codes(&triangle_bbs, triangle_count, &accumulated_bb);
        let bbh_index_buffer = iota(&self.algorithm_resources, triangle_count, 16);
        bitonic_merge_sort(
            &self.algorithm_resources,
            &morton_codes,
            &bbh_index_buffer,
            triangle_count,
        );
        self.build_tree(&triangle_bbs, bbh_index_buffer, triangle_count)
    }

    // calculate triangle morton codes and bbs
    fn calculate_triangle_bbs(
        &self,
        vertex_buffer: &wgpu::Buffer,
        vertex_count: u32,
        index_buffer: &wgpu::Buffer,
        index_count: u32,
    ) -> wgpu::Buffer {
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
            layout: &self.create_bbs_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
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
            compute_pass.set_pipeline(&self.create_bbs_pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);
            compute_pass.dispatch_workgroups(triangle_count, 1, 1);
        }

        let idx = self.renderer.get_queue().submit([encoder.finish()]);
        device.poll(wgpu::Maintain::WaitForSubmissionIndex(idx));

        bb_buffer
    }

    fn accumulate_bbs(&self, bb_buffer: &wgpu::Buffer, triangle_count: u32) -> wgpu::Buffer {
        let device = self.renderer.get_device();
        let bb_buffer_clone = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("bb accumulation"),
            size: triangle_count as u64 * 32,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("accumulate bbs"),
        });
        encoder.copy_buffer_to_buffer(
            bb_buffer,
            0,
            &bb_buffer_clone,
            0,
            triangle_count as u64 * 32,
        );

        let mut offset = 1;
        while offset < triangle_count {
            let params = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("mesh bbh to lines params"),
                contents: bytemuck::cast_slice(&[offset]),
                usage: wgpu::BufferUsages::UNIFORM,
            });
            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("accumulate bbs"),
                layout: &self.accumulate_bbs_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        // TODO: copy this to storage or add storage flag to mesh
                        resource: params.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: bb_buffer_clone.as_entire_binding(),
                    },
                ],
            });
            {
                let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                    label: Some("accumulate bbs"),
                    timestamp_writes: None,
                });
                compute_pass.set_pipeline(&self.accumulate_bbs_pipeline);
                compute_pass.set_bind_group(0, &bind_group, &[]);

                // avoid division by zero
                let thread_count = if offset == 0 {
                    1
                } else {
                    (triangle_count + offset) / (offset * 2)
                };
                compute_pass.dispatch_workgroups(thread_count, 1, 1);
            }

            offset *= 2;
        }

        let idx = self.renderer.get_queue().submit([encoder.finish()]);
        device.poll(wgpu::Maintain::WaitForSubmissionIndex(idx));

        bb_buffer_clone
    }

    fn calculate_morton_codes(
        &self,
        bb_buffer: &wgpu::Buffer,
        triangle_count: u32,
        accumulated_bb: &wgpu::Buffer,
    ) -> wgpu::Buffer {
        let device = self.renderer.get_device();
        let morton_codes = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("morton_codes"),
            size: triangle_count as u64 * 8,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("morton_codes"),
            layout: &self.calculate_morton_codes_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: bb_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: accumulated_bb.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: morton_codes.as_entire_binding(),
                },
            ],
        });
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("morton_codes"),
        });
        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("morton_codes"),
                timestamp_writes: None,
            });
            compute_pass.set_pipeline(&self.calculate_morton_codes_pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);
            compute_pass.dispatch_workgroups(triangle_count, 1, 1);
        }

        let idx = self.renderer.get_queue().submit([encoder.finish()]);
        device.poll(wgpu::Maintain::WaitForSubmissionIndex(idx));

        morton_codes
    }

    // first build leaves, then build tree
    fn build_tree(
        &self,
        triangle_bbs: &wgpu::Buffer,
        bbh_index_buffer: wgpu::Buffer,
        triangle_count: u32,
    ) -> MeshBBH {
        let device = self.renderer.get_device();
        let leaf_count = (triangle_count + MAX_TRIS_PER_LEAF - 1) / MAX_TRIS_PER_LEAF;
        let node_count = leaf_count * 2 - 1;
        let level_count = f32::log2(leaf_count as f32).ceil() as u32 + 1;
        let first_leaf_index = node_count - leaf_count;
        let first_bottom_index = 2u32.pow(level_count - 1) - 1;
        let group_a_count = first_bottom_index - first_leaf_index;
        let group_b_count = leaf_count - group_a_count;

        let tree_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("bbh tree"),
            size: (node_count * NODE_SIZE) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        let params = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("build tree"),
            contents: bytemuck::cast_slice(&[
                MAX_TRIS_PER_LEAF,
                node_count,
                leaf_count,
                triangle_count,
                first_bottom_index,
            ]),
            usage: wgpu::BufferUsages::UNIFORM,
        });
        let init_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("build tree"),
            layout: &self.init_tree_bind_group_layout,
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
                    resource: bbh_index_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: tree_buffer.as_entire_binding(),
                },
            ],
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("build tree"),
        });
        // init tree
        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("init tree"),
                timestamp_writes: None,
            });
            compute_pass.set_pipeline(&self.init_tree_pipeline);
            compute_pass.set_bind_group(0, &init_bind_group, &[]);
            compute_pass.dispatch_workgroups(leaf_count, 1, 1);
        }

        let mut level: i32 = (level_count - 2) as i32;
        let mut thread_count = group_b_count / 2;

        while level >= 0 {
            let offset = u32::pow(2, level as u32) - 1;
            {
                let params = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("build tree params"),
                    contents: bytemuck::cast_slice(&[offset]),
                    usage: wgpu::BufferUsages::UNIFORM,
                });
                let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("build tree"),
                    layout: &self.build_tree_bind_group_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: params.as_entire_binding(),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: tree_buffer.as_entire_binding(),
                        },
                    ],
                });

                let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                    label: Some("build tree"),
                    timestamp_writes: None,
                });
                compute_pass.set_pipeline(&self.build_tree_pipeline);
                compute_pass.set_bind_group(0, &bind_group, &[]);
                compute_pass.dispatch_workgroups(thread_count, 1, 1);
            }
            level -= 1;
            thread_count = 2u32.pow(i32::max(level, 0) as u32);
        }

        let idx = self.renderer.get_queue().submit([encoder.finish()]);
        device.poll(wgpu::Maintain::WaitForSubmissionIndex(idx));

        MeshBBH::new(tree_buffer, bbh_index_buffer, node_count)
    }
}
