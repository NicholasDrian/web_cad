//! BBH generator optimized for fast creation
//! Sorts primitives along morton curve then partitions into tree

use super::MeshBBH;
use crate::utils::create_compute_pipeline;

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
        todo!()
    }

    // calculate triangle morton codes and bbs
    fn calculate_triangle_info(
        vertex_buffer: &wgpu::Buffer,
        vertex_count: u32,
        index_buffer: &wgpu::Buffer,
        index_count: u32,
    ) -> wgpu::Buffer {
        todo!()
    }

    fn build_tree(
        bbh_index_buffer: &wgpu::Buffer,
        bbh_index_count: u32,
        triangle_info_buffer: &wgpu::Buffer,
    ) -> wgpu::Buffer {
        todo!()
    }
}
