pub mod mesh_bbh_generator;

pub struct MeshBBH {
    tree: wgpu::Buffer,
    indices: wgpu::Buffer,
    // for building debug lines
    node_count: u32,
}

impl MeshBBH {
    pub fn new(tree: wgpu::Buffer, indices: wgpu::Buffer, node_count: u32) -> Self {
        Self {
            tree,
            indices,
            node_count,
        }
    }
    pub fn get_tree(&self) -> &wgpu::Buffer {
        &self.tree
    }
    pub fn get_indices(&self) -> &wgpu::Buffer {
        &self.indices
    }
    pub fn get_node_count(&self) -> u32 {
        self.node_count
    }
}
