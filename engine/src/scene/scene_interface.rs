use wasm_bindgen::prelude::*;

use crate::{
    geometry::{
        mesh::{Mesh, MeshVertex},
        polyline::{Polyline, PolylineVertex},
    },
    instance::{Handle, INSTANCES},
};

#[wasm_bindgen]
pub struct Scene {
    instance_handle: Handle,
    scene_handle: Handle,
}
#[wasm_bindgen]
impl Scene {
    pub fn new(instance_handle: Handle, scene_handle: Handle) -> Scene {
        Scene {
            instance_handle,
            scene_handle,
        }
    }

    pub fn get_handle(&self) -> Handle {
        self.scene_handle
    }

    #[wasm_bindgen]
    pub fn add_mesh(&self, positions: &[f32], normals: &[f32], indices: &[u32]) {
        let mut verts: Vec<MeshVertex> = Vec::new();
        for i in 0..positions.len() / 3 {
            verts.push(MeshVertex {
                position: [
                    positions[i * 3],
                    positions[i * 3 + 1],
                    positions[i * 3 + 2],
                    1.0,
                ],
                normal: [normals[i * 3], normals[i * 3 + 1], normals[i * 3 + 2], 0.0],
            })
        }

        // TODO: why do i need two gets????
        // I currently hate the borrow checker
        let mesh = Mesh::new(
            INSTANCES
                .lock()
                .unwrap()
                .get_mut(&self.instance_handle)
                .unwrap()
                .get_renderer()
                .get_device(),
            &verts[..],
            indices,
        );

        INSTANCES
            .lock()
            .unwrap()
            .get_mut(&self.instance_handle)
            .unwrap()
            .get_scene_mut(self.scene_handle)
            .add_mesh(mesh);
    }

    #[wasm_bindgen]
    pub fn add_polyline(&self, positions: &[f32]) {
        let mut verts: Vec<PolylineVertex> = Vec::new();
        for i in 0..positions.len() / 3 {
            verts.push(PolylineVertex {
                position: [
                    positions[i * 3],
                    positions[i * 3 + 1],
                    positions[i * 3 + 2],
                    1.0,
                ],
            });
        }
        let polyline = Polyline::new(
            INSTANCES
                .lock()
                .unwrap()
                .get_mut(&self.instance_handle)
                .unwrap()
                .get_renderer()
                .get_device(),
            &verts[..],
        );

        INSTANCES
            .lock()
            .unwrap()
            .get_mut(&self.instance_handle)
            .unwrap()
            .get_scene_mut(self.scene_handle)
            .add_polyline(polyline);
    }
}
