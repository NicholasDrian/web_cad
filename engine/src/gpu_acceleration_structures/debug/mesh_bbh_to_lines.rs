use crate::{geometry::lines::Lines, gpu_acceleration_structures::mesh_bbh::MeshBBH};

// Rebuilding pipeline and such every call.
// Expensive, but this is only for debug so its chill
pub fn mesh_bbh_to_lines(mesh_bbh: &MeshBBH) -> Lines {
    let node_count = mesh_bbh.get_node_count();
    let tree_buffer = mesh_bbh.get_tree();

    todo!()
}
