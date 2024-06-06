//! BBH generator optimized for fast creation

use super::MeshBBH;

pub struct MeshBBHGeneratorFastBuild {
    renderer: std::rc::Rc<crate::render::renderer::Renderer>,
    algorithm_resources: std::rc::Rc<crate::gpu_algorithms::AlgorithmResources>,
}

impl MeshBBHGeneratorFastBuild {
    pub fn new(
        renderer: std::rc::Rc<crate::render::renderer::Renderer>,
        algorithm_resources: std::rc::Rc<crate::gpu_algorithms::AlgorithmResources>,
    ) -> Self {
        Self {
            renderer,
            algorithm_resources,
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

    fn calculate_morton_codes(
        vertex_buffer: &wgpu::Buffer,
        vertex_count: u32,
        index_buffer: &wgpu::Buffer,
        index_count: u32,
    ) -> wgpu::Buffer {
        todo!()
    }
}
