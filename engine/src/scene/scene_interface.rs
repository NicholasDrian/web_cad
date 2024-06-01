use wasm_bindgen::prelude::*;

use crate::{
    geometry::{
        curve::Curve,
        lines::{Lines, LinesVertex},
        mesh::{Mesh, MeshVertex},
        polyline::{Polyline, PolylineVertex},
        surface::Surface,
        GeometryId,
    },
    gpu_acceleration_structures::debug::mesh_bbh_to_lines::mesh_bbh_to_lines,
    instance::Handle,
    math::linear_algebra::{vec3::Vec3, vec4::Vec4},
    utils::get_instance_mut,
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
    pub fn get_instance_handle(&self) -> Handle {
        self.instance_handle
    }

    #[wasm_bindgen]
    pub async fn add_mesh(
        &self,
        positions: &[f32],
        normals: &[f32],
        indices: &[u32],
    ) -> GeometryId {
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
        let mut mesh = Mesh::new(
            get_instance_mut!(&self.instance_handle)
                .get_renderer()
                .clone(),
            &verts[..],
            indices,
        );
        let bbh = get_instance_mut!(&self.instance_handle)
            .get_mesh_bbh_generator()
            .generate_mesh_bbh(&mesh)
            .await;

        // For debug
        let renderer = get_instance_mut!(&self.instance_handle).get_renderer();
        get_instance_mut!(&self.instance_handle)
            .get_scene_mut(self.scene_handle)
            .add_lines(mesh_bbh_to_lines(renderer, &bbh));

        mesh.add_bbh(bbh);

        get_instance_mut!(&self.instance_handle)
            .get_scene_mut(self.scene_handle)
            .add_mesh(mesh)
    }

    #[wasm_bindgen]
    pub fn add_polyline(&self, vertices: &[f32]) -> GeometryId {
        let mut verts: Vec<PolylineVertex> = Vec::new();
        for i in 0..vertices.len() / 3 {
            verts.push(PolylineVertex {
                position: [
                    vertices[i * 3],
                    vertices[i * 3 + 1],
                    vertices[i * 3 + 2],
                    1.0,
                ],
            });
        }

        let polyline = Polyline::new(
            get_instance_mut!(&self.instance_handle).get_renderer(),
            &verts[..],
        );

        get_instance_mut!(&self.instance_handle)
            .get_scene_mut(self.scene_handle)
            .add_polyline(polyline)
    }

    #[wasm_bindgen]
    pub fn add_lines(&self, vertices: &[f32], indices: &[u32]) -> GeometryId {
        let mut verts: Vec<LinesVertex> = Vec::new();
        for i in 0..vertices.len() / 3 {
            verts.push(LinesVertex {
                position: [
                    vertices[i * 3],
                    vertices[i * 3 + 1],
                    vertices[i * 3 + 2],
                    1.0,
                ],
            });
        }

        let lines = Lines::new(
            get_instance_mut!(&self.instance_handle).get_renderer(),
            &verts[..],
            indices,
        );

        get_instance_mut!(&self.instance_handle)
            .get_scene_mut(self.scene_handle)
            .add_lines(lines)
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
        let mut weighted_control_points: Vec<Vec4> = Vec::new();
        let weights = if weights.len() == 0 {
            &vec![1.0; controls.len()][..]
        } else {
            weights
        };
        for i in 0..controls.len() / 3 {
            weighted_control_points.push(Vec4 {
                x: controls[i * 3] * weights[i],
                y: controls[i * 3 + 1] * weights[i],
                z: controls[i * 3 + 2] * weights[i],
                w: weights[i],
            });
        }

        // TODO: this warning indicates performance issues
        let curve = Curve::new(
            get_instance_mut!(&self.instance_handle).get_curve_sampler(),
            degree,
            weighted_control_points,
            knots,
        );

        get_instance_mut!(&self.instance_handle)
            .get_scene_mut(self.scene_handle)
            .add_curve(curve)
    }
    /// Controls should be row major, and U major
    ///
    /// The layout is as follows:
    ///
    ///     U ----->
    ///   V 0, 1, 2,
    ///   | 3, 4, 5,
    ///   v 6, 7, 8,
    ///
    /// In the above diagram, each number represents a point, 3 floats
    ///
    /// Leave weights and knots empty for default values
    ///

    #[wasm_bindgen]
    pub fn add_surface(
        &self,
        degree_u: u32,
        degree_v: u32,
        controls: &[f32],
        control_count_u: u32,
        control_count_v: u32,
        weights: &[f32],
        knots_u: &[f32],
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
        let surface = Surface::new(
            get_instance_mut!(&self.instance_handle).get_surface_sampler(),
            control_count_u,
            control_count_v,
            degree_u,
            degree_v,
            control_points,
            weights,
            knots_u,
            knots_v,
        );

        get_instance_mut!(&self.instance_handle)
            .get_scene_mut(self.scene_handle)
            .add_surface(surface)
    }

    #[wasm_bindgen]
    pub fn rotate_geometry(&self, geometry_id: u32, center: &[f32], axis: &[f32], radians: f32) {
        get_instance_mut!(&self.instance_handle)
            .get_scene_mut(self.scene_handle)
            .rotate_geometry(geometry_id, center, axis, radians);
    }

    #[wasm_bindgen]
    pub fn delete_geometry(&self, geometry_id: u32) {
        get_instance_mut!(&self.instance_handle)
            .get_scene_mut(self.scene_handle)
            .delete_geometry(geometry_id);
    }

    // NOTE: this will break once adaptive sampling is added
    /// Control point count cannot change in either dimension.
    #[wasm_bindgen]
    pub fn update_surface_params(
        &self,
        id: GeometryId,
        degree_u: u32,
        degree_v: u32,
        controls: &[f32],
        weights: &[f32],
        knots_u: &[f32],
        knots_v: &[f32],
    ) {
        if let Some(surface) = get_instance_mut!(&self.instance_handle)
            .get_scene_mut(self.scene_handle)
            .get_surfaces_mut()
            .get_mut(&id)
        {
            let mut control_points: Vec<Vec3> = Vec::new();
            for i in 0..controls.len() / 3 {
                control_points.push(Vec3 {
                    x: controls[i * 3],
                    y: controls[i * 3 + 1],
                    z: controls[i * 3 + 2],
                });
            }
            surface.update_params(
                degree_u,
                degree_v,
                control_points,
                weights,
                knots_u,
                knots_v,
            )
        }
    }
}
