use std::rc::Rc;

use wgpu::util::DeviceExt;

use crate::geometry::mesh::Mesh;
use crate::gpu_algorithms::{iota::iota, AlgorithmResources};
use crate::render::renderer::Renderer;
use crate::utils::create_compute_pipeline;

const SPLIT_CANDIDATE_COUNT: u32 = 8;
const _: () = assert!(SPLIT_CANDIDATE_COUNT <= 32);

pub struct MeshBBHGenerator {
    renderer: Rc<Renderer>,
    algorithm_resources: Rc<AlgorithmResources>,

    triangle_info_buffer_generator_bind_group_layout: wgpu::BindGroupLayout,
    triangle_info_buffer_generator_pipeline: wgpu::ComputePipeline,

    split_bind_group_layout: wgpu::BindGroupLayout,
    split_pipeline: wgpu::ComputePipeline,

    update_lr_bind_group_layout: wgpu::BindGroupLayout,
    update_lr_pipeline: wgpu::ComputePipeline,
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
                    // segments
                    crate::utils::compute_buffer_bind_group_layout_entry(0, true),
                    // bbh index buffer
                    crate::utils::compute_buffer_bind_group_layout_entry(1, true),
                    // triangle info buffer
                    crate::utils::compute_buffer_bind_group_layout_entry(2, true),
                    // split candidates
                    crate::utils::compute_buffer_bind_group_layout_entry(3, false),
                ],
            });
        let update_lr_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("update lr"),
                entries: &[
                    // params
                    crate::utils::compute_uniform_bind_group_layout_entry(0),
                    // segments
                    crate::utils::compute_buffer_bind_group_layout_entry(1, true),
                    // split candidates
                    crate::utils::compute_buffer_bind_group_layout_entry(2, true),
                    // bbh index buffer
                    crate::utils::compute_buffer_bind_group_layout_entry(3, false),
                    // triangle info buffer
                    crate::utils::compute_buffer_bind_group_layout_entry(4, false),
                ],
            });

        let triangle_info_buffer_generator_pipeline = create_compute_pipeline(
            device,
            "gen triangle info buffer",
            include_str!("gen_triangle_info_buffer.wgsl"),
            &triangle_info_buffer_generator_bind_group_layout,
            "generate_bb_buffer",
        );
        let split_pipeline = create_compute_pipeline(
            device,
            "split finder",
            include_str!("split.wgsl"),
            &split_bind_group_layout,
            "find_splits",
        );
        let update_lr_pipeline = create_compute_pipeline(
            device,
            "update lr",
            include_str!("reorder_indices.wgsl"),
            &update_lr_bind_group_layout,
            "update_lr",
        );

        Self {
            renderer,
            algorithm_resources,
            triangle_info_buffer_generator_bind_group_layout,
            triangle_info_buffer_generator_pipeline,
            split_bind_group_layout,
            split_pipeline,
            update_lr_bind_group_layout,
            update_lr_pipeline,
        }
    }

    pub fn create_bbh(&self, mesh: &Mesh) -> wgpu::Buffer {
        let triangle_info_buffer = self.create_triangle_info_buffer(
            mesh.get_vertex_buffer(),
            mesh.get_index_buffer(),
            mesh.get_index_count() / 3,
        );

        let triangle_count = mesh.get_index_count() / 3;
        let bbh_index_buffer = iota(&self.algorithm_resources, triangle_count, 16);

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
            let (previous_level, previous_level_length) = levels.last().unwrap();
            let splits = self.split(
                &triangle_info_buffer,
                previous_level,
                *previous_level_length,
                &bbh_index_buffer,
            );
            self.update_lr(
                &triangle_info_buffer,
                previous_level,
                *previous_level_length,
                &splits,
                &bbh_index_buffer,
            );
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
        triangle_count: u32,
    ) -> wgpu::Buffer {
        let device = self.renderer.get_device();

        // Check this size!!!!!!
        let triangle_info_size = 64;
        let triangle_info_buffer = device.create_buffer(&wgpu::BufferDescriptor {
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
                    resource: triangle_info_buffer.as_entire_binding(),
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
            compute_pass.dispatch_workgroups(triangle_count / 3, 1, 1);
        }

        let idx = self.renderer.get_queue().submit([encoder.finish()]);
        device.poll(wgpu::Maintain::WaitForSubmissionIndex(idx));

        triangle_info_buffer
    }

    // Take a bunch of segments, and find possible split points and direction.
    // Evaluate randomly selected splits and report their quality.
    // TODO: seed the randomness using time param
    fn split(
        &self,
        triangle_info_buffer: &wgpu::Buffer,
        segments: &wgpu::Buffer,
        segment_count: u32,
        bbh_index_buffer: &wgpu::Buffer,
    ) -> wgpu::Buffer {
        let device = self.renderer.get_device();
        let queue = self.renderer.get_queue();

        let split_candidates: wgpu::Buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("mesh bbh split candidates"),
            // segment count * candidates_per_segment * (x,y,z,quality, quality, quality) * size_of(f32)
            size: (segment_count * SPLIT_CANDIDATE_COUNT * 6 * 4) as u64,
            usage: wgpu::BufferUsages::STORAGE,
            mapped_at_creation: false,
        });

        let bind_group: wgpu::BindGroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("mesh bbh split"),
            layout: &self.split_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: triangle_info_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: segments.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: bbh_index_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: split_candidates.as_entire_binding(),
                },
            ],
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("mesh bbh split"),
        });

        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("mesh bbh split"),
                timestamp_writes: None,
            });

            compute_pass.set_pipeline(&self.split_pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);
            compute_pass.dispatch_workgroups(segment_count, SPLIT_CANDIDATE_COUNT, 1);
        }

        let idx = queue.submit([encoder.finish()]);
        device.poll(wgpu::Maintain::WaitForSubmissionIndex(idx));

        split_candidates
    }

    fn update_lr(
        &self,
        triangle_info_buffer: &wgpu::Buffer,
        segments: &wgpu::Buffer,
        segment_count: u32,
        split_candidates: &wgpu::Buffer,
        bbh_index_buffer: &wgpu::Buffer,
    ) {
        // TODO: could probably reorder indices in this stage too
        let device = self.renderer.get_device();
        let queue = self.renderer.get_queue();

        let params = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("update lr params"),
            contents: bytemuck::cast_slice(&[SPLIT_CANDIDATE_COUNT]),
            usage: wgpu::BufferUsages::UNIFORM,
        });

        let bind_group: wgpu::BindGroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("mesh bbh split"),
            layout: &self.update_lr_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: params.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: segments.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: bbh_index_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: split_candidates.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: triangle_info_buffer.as_entire_binding(),
                },
            ],
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("mesh bbh split"),
        });

        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("mesh bbh split"),
                timestamp_writes: None,
            });

            compute_pass.set_pipeline(&self.update_lr_pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);
            compute_pass.dispatch_workgroups(segment_count, 1, 1);
        }

        let idx = queue.submit([encoder.finish()]);
        device.poll(wgpu::Maintain::WaitForSubmissionIndex(idx));
    }

    // reorder_indices and add nodes.
    fn build_bbh() {
        todo!()
    }
}
