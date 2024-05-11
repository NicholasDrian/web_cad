use wasm_bindgen::prelude::*;

use crate::{
    geometry::mesh::{Mesh, MeshVertex},
    instance::{Handle, INSTANCES},
    math::linear_algebra::vec3::Vec3,
};

#[wasm_bindgen]
pub fn add_mesh(
    instance_handle: Handle,
    scene_handle: Handle,
    positions: Vec<Vec3>,
    normals: Vec<Vec3>,
    indices: Vec<u32>,
) {
    let verts: Vec<MeshVertex> = positions
        .iter()
        .zip(normals.iter())
        .map(|(pos, norm)| MeshVertex {
            position: (*pos).into(),
            normal: (*norm).into(),
        })
        .collect();

    // TODO: why do i need two gets????
    // I currently hate the borrow checker
    let mesh = Mesh::new(
        INSTANCES
            .lock()
            .unwrap()
            .get_mut(&instance_handle)
            .unwrap()
            .get_renderer()
            .get_device(),
        &verts[..],
        &indices[..],
    );

    INSTANCES
        .lock()
        .unwrap()
        .get_mut(&instance_handle)
        .unwrap()
        .get_scene_mut(scene_handle)
        .add_mesh(mesh);
}
