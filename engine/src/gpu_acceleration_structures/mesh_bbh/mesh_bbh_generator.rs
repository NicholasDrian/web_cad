use std::rc::Rc;

use crate::{
    geometry::mesh::Mesh,
    gpu_algorithms::{iota::iota, AlgorithmResources},
    render::renderer::Renderer,
    utils::create_compute_pipeline,
};

use super::mesh_bbh::MeshBBH;

pub struct MeshBBHGenerator {
    renderer: Rc<Renderer>,
    algorithm_resources: Rc<AlgorithmResources>,

    create_triangle_bbs_bind_group_layout: wgpu::BindGroupLayout,
    create_triangle_bbs_pipeline: wgpu::ComputePipeline,
}

impl MeshBBHGenerator {
    pub fn new(renderer: Rc<Renderer>, algorithm_resources: Rc<AlgorithmResources>) -> Self {
        let device = renderer.get_device();
        let create_triangle_bbs_bind_group_layout =
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
        let create_triangle_bbs_pipeline = create_compute_pipeline(
            device,
            "gen triangle info buffer",
            include_str!("create_triangle_bbs.wgsl"),
            &create_triangle_bbs_bind_group_layout,
            "generate_bb_buffer",
        );
        Self {
            renderer,
            algorithm_resources,
            create_triangle_bbs_bind_group_layout,
            create_triangle_bbs_pipeline,
        }
    }
    pub fn generate_mesh_bbh(&self, mesh: &Mesh) -> MeshBBH {
        let triangle_count = mesh.get_index_count() / 3;
        let triangle_info: wgpu::Buffer = self.create_triangle_bbs(mesh);
        let index_buffer = iota(&self.algorithm_resources, triangle_count, 16);
        let tree_buffer = self.init_tree_buffer(mesh);
        let mut input: (u32, u32) = (0, 1);
        loop {
            // TODO: remove this in favor of bottom up approach
            self.build_bbs(&tree_buffer, &index_buffer, &triangle_info, input);

            // prefix sum of number of nodes with children
            let (prefix_sum, total) = self.prefix_sum(&tree_buffer, input);
            if (total == 0) {
                // Input is all leaves. were done
                break;
            }

            self.set_child_pointers(&tree_buffer, &prefix_sum, input);

            let split_evaluations = self.split_evaluations(&tree_buffer, &index_buffer, input);
            self.build_next_level(
                &tree_buffer,
                &index_buffer,
                &split_evaluations,
                &prefix_sum,
                input,
            );

            input = (input.1, input.1 + total);
        }

        MeshBBH::new(tree_buffer, index_buffer)
    }

    // Good ez paralelism
    fn create_triangle_bbs(&self, mesh: &Mesh) -> wgpu::Buffer {
        let device = self.renderer.get_device();

        let triangle_count = mesh.get_index_count() / 3;
        let triangle_info_size = 16 * 3;
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
                    resource: triangle_info_buffer.as_entire_binding(),
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

        triangle_info_buffer
    }

    // EZ
    fn init_tree_buffer(&self, mesh: &Mesh) -> wgpu::Buffer {
        todo!()
    }

    // I got this paralell
    fn prefix_sum(&self, tree: &wgpu::Buffer, range: (u32, u32)) -> (wgpu::Buffer, u32) {
        todo!()
    }

    // TODO: replace this with bottum up version
    fn build_bbs(
        &self,
        tree: &wgpu::Buffer,
        indices: &wgpu::Buffer,
        triangle_info: &wgpu::Buffer,
        range: (u32, u32),
    ) {
        todo!()
    }

    // Wicked fast
    fn set_child_pointers(
        &self,
        tree: &wgpu::Buffer,
        prefix_sum: &wgpu::Buffer,
        input: (u32, u32),
    ) {
        todo!()
    }

    // TODO: make more paralel
    fn split_evaluations(
        &self,
        tree_buffer: &wgpu::Buffer,
        index_buffer: &wgpu::Buffer,
        input: (u32, u32),
    ) -> wgpu::Buffer {
        todo!()
    }

    // reorder indices and write out next level
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
