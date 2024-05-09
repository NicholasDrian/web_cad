use std::collections::HashMap;

use crate::geometry::{
    curve::Curve, geometry_id::GeometryId, mesh::Mesh, polyline::Polyline, surface::Surface,
};

pub struct Scene {
    curves: HashMap<GeometryId, Curve>,
    surfaces: HashMap<GeometryId, Surface>,
    polylines: HashMap<GeometryId, Polyline>,
    meshes: HashMap<GeometryId, Mesh>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
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
}
