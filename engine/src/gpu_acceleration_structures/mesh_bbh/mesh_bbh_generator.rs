use std::rc::Rc;

use crate::{gpu_algorithms::AlgorithmResources, render::renderer::Renderer};

use super::{
    generator_fast_build::MeshBBHGeneratorFastBuild,
    generator_fast_build_2::MeshBBHGeneratorFastBuild2,
    generator_fast_trace::MeshBBHGeneratorFastTrace, MeshBBH,
};

pub struct MeshBBHGenerator {
    fast_trace_generator: MeshBBHGeneratorFastTrace,
    fast_build_generator: MeshBBHGeneratorFastBuild,
    fast_build_generator_2: MeshBBHGeneratorFastBuild2,
}

impl MeshBBHGenerator {
    pub fn new(renderer: Rc<Renderer>, algorithm_resources: Rc<AlgorithmResources>) -> Self {
        let fast_trace_generator =
            MeshBBHGeneratorFastTrace::new(renderer.clone(), algorithm_resources.clone());
        let fast_build_generator =
            MeshBBHGeneratorFastBuild::new(renderer.clone(), algorithm_resources.clone());
        let fast_build_generator_2 = MeshBBHGeneratorFastBuild2::new(renderer, algorithm_resources);
        Self {
            fast_trace_generator,
            fast_build_generator,
            fast_build_generator_2,
        }
    }
    pub async fn generate_mesh_bbh_fast_trace(
        &self,
        vertex_buffer: &wgpu::Buffer,
        vertex_count: u32,
        index_buffer: &wgpu::Buffer,
        index_count: u32,
    ) -> MeshBBH {
        self.fast_trace_generator
            .generate_mesh_bbh(vertex_buffer, vertex_count, index_buffer, index_count)
            .await
    }

    pub async fn generate_mesh_bbh_fast_build(
        &self,
        vertex_buffer: &wgpu::Buffer,
        vertex_count: u32,
        index_buffer: &wgpu::Buffer,
        index_count: u32,
    ) -> MeshBBH {
        self.fast_build_generator.generate_mesh_bbh(
            vertex_buffer,
            vertex_count,
            index_buffer,
            index_count,
        )
    }

    pub fn generate_mesh_bbh_fast_build_2(
        &self,
        vertex_buffer: &wgpu::Buffer,
        vertex_count: u32,
        index_buffer: &wgpu::Buffer,
        index_count: u32,
    ) -> MeshBBH {
        self.fast_build_generator_2.generate_mesh_bbh(
            vertex_buffer,
            vertex_count,
            index_buffer,
            index_count,
        )
    }
}
