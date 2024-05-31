use std::rc::Rc;

use crate::{
    geometry::mesh::Mesh,
    gpu_algorithms::{iota::iota, AlgorithmResources},
    render::renderer::Renderer,
};

use super::mesh_bbh::MeshBBH;

pub struct MeshBBHGenerator {
    renderer: Rc<Renderer>,
    algorithm_resources: Rc<AlgorithmResources>,
}

impl MeshBBHGenerator {
    pub fn generate_mesh_bbh(&self, mesh: &Mesh) -> MeshBBH {
        let triangle_count = mesh.get_index_count() / 3;
        let triangle_info: wgpu::Buffer = self.create_triangle_info(mesh);
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

    fn create_triangle_info(&self, mesh: &Mesh) -> wgpu::Buffer {
        todo!()
    }

    fn init_tree_buffer(&self, mesh: &Mesh) -> wgpu::Buffer {
        todo!()
    }

    fn prefix_sum(&self, tree: &wgpu::Buffer, range: (u32, u32)) -> (wgpu::Buffer, u32) {
        todo!()
    }

    fn build_bbs(
        &self,
        tree: &wgpu::Buffer,
        indices: &wgpu::Buffer,
        triangle_info: &wgpu::Buffer,
        range: (u32, u32),
    ) {
        todo!()
    }

    fn set_child_pointers(
        &self,
        tree: &wgpu::Buffer,
        prefix_sum: &wgpu::Buffer,
        input: (u32, u32),
    ) {
        todo!()
    }

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
