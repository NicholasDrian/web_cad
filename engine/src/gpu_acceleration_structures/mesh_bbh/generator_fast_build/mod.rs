//! BBH generator optimized for fast creation
//! Sorts primitives along morton curve then partitions into tree

use super::MeshBBH;
use crate::{
    geometry::mesh::MeshVertex,
    gpu_algorithms::{bitonic_merge_sort::bitonic_merge_sort, iota::iota},
    utils::create_compute_pipeline,
};

pub struct MeshBBHGeneratorFastBuild {
    renderer: std::rc::Rc<crate::render::renderer::Renderer>,
    algorithm_resources: std::rc::Rc<crate::gpu_algorithms::AlgorithmResources>,

    create_triangle_info_bind_group_layout: wgpu::BindGroupLayout,
    create_triangle_info_pipeline: wgpu::ComputePipeline,

    accumulate_bbs_bind_group_layout: wgpu::BindGroupLayout,
    accumulate_bbs_pipeline: wgpu::ComputePipeline,

    build_tree_bind_group_layout: wgpu::BindGroupLayout,
    build_tree_pipeline: wgpu::ComputePipeline,
}

impl MeshBBHGeneratorFastBuild {
    pub fn new(
        renderer: std::rc::Rc<crate::render::renderer::Renderer>,
        algorithm_resources: std::rc::Rc<crate::gpu_algorithms::AlgorithmResources>,
    ) -> Self {
        let device = renderer.get_device();
        let create_triangle_info_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("create_triangle info buffer"),
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
        let build_tree_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("build tree"),
                entries: &[
                    // params
                    crate::utils::compute_uniform_bind_group_layout_entry(0),
                    // bbh index buffer
                    crate::utils::compute_buffer_bind_group_layout_entry(1, true),
                    // Index buffer
                    crate::utils::compute_buffer_bind_group_layout_entry(2, true),
                    // Triangle info buffer
                    crate::utils::compute_buffer_bind_group_layout_entry(3, true),
                    // tree buffer
                    crate::utils::compute_buffer_bind_group_layout_entry(4, false),
                ],
            });
        let create_triangle_info_pipeline = create_compute_pipeline(
            device,
            "create triangle info",
            include_str!("create_triangle_info_pipeline.wgsl"),
            &create_triangle_info_bind_group_layout,
            "main",
        );
        let accumulate_bbs_pipeline = create_compute_pipeline(
            device,
            "accumulate bbs",
            include_str!("accumulate_bbs.wgsl"),
            &accumulate_bbs_bind_group_layout,
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
            create_triangle_info_bind_group_layout,
            create_triangle_info_pipeline,
            accumulate_bbs_bind_group_layout,
            accumulate_bbs_pipeline,
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
        self.build_tree(
            &triangle_bbs,
            &morton_codes,
            &bbh_index_buffer,
            triangle_count,
        )
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
        let triangle_info_buffer = device.create_buffer(&wgpu::BufferDescriptor {
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
            layout: &self.create_triangle_info_bind_group_layout,
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
                    resource: triangle_info_buffer.as_entire_binding(),
                },
            ],
        });
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("create triangle_info_buffer"),
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

            compute_pass.set_pipeline(&self.create_triangle_info_pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);
            compute_pass.dispatch_workgroups(triangle_count, 1, 1);
        }

        let idx = self.renderer.get_queue().submit([encoder.finish()]);
        device.poll(wgpu::Maintain::WaitForSubmissionIndex(idx));

        triangle_info_buffer
    }

    fn accumulate_bbs(&self, bb_buffer: &wgpu::Buffer, triangle_count: u32) -> wgpu::Buffer {
        todo!()
    }

    fn calculate_morton_codes(
        &self,
        bb_buffer: &wgpu::Buffer,
        triangle_count: u32,
        accumulated_bb: &wgpu::Buffer,
    ) -> wgpu::Buffer {
        todo!()
    }

    fn build_tree(
        &self,
        triangle_bbs: &wgpu::Buffer,
        morton_codes: &wgpu::Buffer,
        bbh_index_buffer: &wgpu::Buffer,
        triangle_count: u32,
    ) -> MeshBBH {
        todo!()
    }
}
