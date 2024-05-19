use wasm_bindgen::prelude::*;

use crate::{
    geometry::{
        curve::Curve,
        geometry::GeometryId,
        mesh::{Mesh, MeshVertex},
        polyline::{Polyline, PolylineVertex},
        surface::Surface,
    },
    instance::{Handle, INSTANCES},
    math::linear_algebra::vec3::Vec3,
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
    pub fn add_mesh(&self, positions: &[f32], normals: &[f32], indices: &[u32]) -> GeometryId {
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
            &INSTANCES
                .lock()
                .unwrap()
                .get_mut(&self.instance_handle)
                .unwrap()
                .get_renderer(),
            &verts[..],
            indices,
        );

        INSTANCES
            .lock()
            .unwrap()
            .get_mut(&self.instance_handle)
            .unwrap()
            .get_scene_mut(self.scene_handle)
            .add_mesh(mesh)
    }

    #[wasm_bindgen]
    pub fn add_polyline(&self, positions: &[f32]) -> GeometryId {
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
            .add_polyline(polyline)
    }

    /// Controls ar packed into a flat list of floats for performance and ease
    /// The structure of the controls is as follows:
    ///     point0.x, point0.y, point0.z,
    ///     point1.x, point1.y, point1.z,
    ///     point2.x...
    ///
    /// This function blocks until GPU is done creating curve
    ///
    #[wasm_bindgen]
    pub fn add_curve(
        &self,
        degree: u32,
        controls: &[f32],
        // Leave empty for default values
        weights: &[f32],
        // Leave empty for default values
        knots: &[f32],
    ) -> GeometryId {
        let mut control_points: Vec<Vec3> = Vec::new();
        for i in 0..controls.len() / 3 {
            control_points.push(Vec3 {
                x: controls[i * 3],
                y: controls[i * 3 + 1],
                z: controls[i * 3 + 2],
            });
        }

        // TODO: this warning indicates performance issues
        let curve = Curve::new(
            INSTANCES
                .lock()
                .unwrap()
                .get_mut(&self.instance_handle)
                .unwrap()
                .get_curve_sampler(),
            degree,
            control_points,
            weights,
            knots,
        );

        INSTANCES
            .lock()
            .unwrap()
            .get_mut(&self.instance_handle)
            .unwrap()
            .get_scene_mut(self.scene_handle)
            .add_curve(curve)
    }

    /// Controls should be row major, and U major
    /// The layout is as follows:
    ///
    ///     U ----->
    ///   V 0, 1, 2,
    ///   | 3, 4, 5,
    ///   v 6, 7, 8,
    ///
    /// In the above diagram, each number represents 3 floats, x, y, and z,
    ///
    /// Weights have the same layout.
    ///
    /// This function blocks until GPU is done creating surface.
    ///
    #[wasm_bindgen]
    pub fn add_surface(
        &self,
        degree_u: u32,
        degree_v: u32,
        controls: &[f32],
        control_count_u: u32,
        control_count_v: u32,
        // Leave empty for default values
        weights: &[f32],
        // Leave empty for default values
        knots_u: &[f32],
        // Leave empty for default values
        knots_v: &[f32],
    ) -> GeometryId {
        let mut control_points: Vec<Vec3> = Vec::new();
        for i in 0..controls.len() / 3 {
            control_points.push(Vec3 {
                x: controls[i * 3],
                y: controls[i * 3 + 1],
                z: controls[i * 3 + 2],
            });
        }
        // TODO: this warning indicates performance issues
        let surface = Surface::new(
            INSTANCES
                .lock()
                .unwrap()
                .get_mut(&self.instance_handle)
                .unwrap()
                .get_surface_sampler(),
            control_count_u,
            control_count_v,
            degree_u,
            degree_v,
            control_points,
            weights,
            knots_u,
            knots_v,
        );

        INSTANCES
            .lock()
            .unwrap()
            .get_mut(&self.instance_handle)
            .unwrap()
            .get_scene_mut(self.scene_handle)
            .add_surface(surface)
    }
}
