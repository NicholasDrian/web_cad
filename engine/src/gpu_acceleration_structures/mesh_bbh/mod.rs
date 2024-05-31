pub mod mesh_bbh_generator;

pub struct MeshBBH {
    tree: wgpu::Buffer,
    indices: wgpu::Buffer,
}

impl MeshBBH {
    pub fn new(tree: wgpu::Buffer, indices: wgpu::Buffer) -> Self {
        Self { tree, indices }
    }
    pub fn get_tree(&self) -> &wgpu::Buffer {
        &self.tree
    }
    pub fn get_indices(&self) -> &wgpu::Buffer {
        &self.indices
    }
}
