use std::collections::HashMap;

use crate::geometry::{
    curve::Curve,
    geometry::{new_geometry_id, GeometryId},
    mesh::Mesh,
    polyline::Polyline,
    surface::Surface,
};

pub struct SceneInternal {
    curves: HashMap<GeometryId, Curve>,
    surfaces: HashMap<GeometryId, Surface>,
    polylines: HashMap<GeometryId, Polyline>,
    meshes: HashMap<GeometryId, Mesh>,
}

impl SceneInternal {
    pub fn new() -> SceneInternal {
        SceneInternal {
            curves: HashMap::new(),
            surfaces: HashMap::new(),
            polylines: HashMap::new(),
            meshes: HashMap::new(),
        }
    }

    pub fn get_curves(&self) -> &HashMap<GeometryId, Curve> {
        &self.curves
    }
    pub fn get_surfaces(&self) -> &HashMap<GeometryId, Surface> {
        &self.surfaces
    }
    pub fn get_polylines(&self) -> &HashMap<GeometryId, Polyline> {
        &self.polylines
    }
    pub fn get_meshes(&self) -> &HashMap<GeometryId, Mesh> {
        &self.meshes
    }
    pub fn add_curve(&mut self, curve: Curve) {
        let id = new_geometry_id();
        self.curves.insert(id, curve);
    }
    pub fn add_surface(&mut self, surface: Surface) {
        let id = new_geometry_id();
        self.surfaces.insert(id, surface);
    }
    pub fn add_polyline(&mut self, polyline: Polyline) {
        let id = new_geometry_id();
        self.polylines.insert(id, polyline);
    }
    pub fn add_mesh(&mut self, mesh: Mesh) {
        let id = new_geometry_id();
        self.meshes.insert(id, mesh);
    }
}
