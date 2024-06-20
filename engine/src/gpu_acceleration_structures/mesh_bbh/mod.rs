use crate::math::linear_algebra::vec3::Vec3;

pub mod generator_fast_build;
pub mod generator_fast_build_2;
pub mod generator_fast_trace;
pub mod mesh_bbh_generator;

pub struct MeshBBH {
    tree: wgpu::Buffer,
    indices: wgpu::Buffer,
    // for building debug lines
    node_count: u32,
}

// used for debug print
#[allow(dead_code)]
#[repr(C)]
#[derive(Copy, Clone, bytemuck::NoUninit)]
struct MeshBBHNode {
    pub min_corner: Vec3,
    pub max_corner: Vec3,
    pub l: u32,
    pub r: u32,
    pub left_child: u32,
}

impl std::fmt::Debug for MeshBBHNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Node \n range:({},{})\n min:{} max:{}\n left_child:{}",
            self.l, self.r, self.min_corner, self.max_corner, self.left_child
        )
    }
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

    // for debug print
    pub fn get_node_count(&self) -> u32 {
        self.node_count
    }
}
