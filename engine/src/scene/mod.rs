pub mod scene_interface;

use std::collections::HashMap;

use crate::geometry::{
    curve::Curve, lines::Lines, mesh::Mesh, new_geometry_id, polyline::Polyline, surface::Surface,
    Geometry, GeometryId,
};

pub struct SceneInternal {
    curves: HashMap<GeometryId, Curve>,
    surfaces: HashMap<GeometryId, Surface>,
    polylines: HashMap<GeometryId, Polyline>,
    meshes: HashMap<GeometryId, Mesh>,
    lines: HashMap<GeometryId, Lines>,
}

impl SceneInternal {
    pub fn new() -> SceneInternal {
        SceneInternal {
            curves: HashMap::new(),
            surfaces: HashMap::new(),
            polylines: HashMap::new(),
            meshes: HashMap::new(),
            lines: HashMap::new(),
        }
    }

    pub fn get_curves_mut(&mut self) -> &mut HashMap<GeometryId, Curve> {
        &mut self.curves
    }
    pub fn get_lines_mut(&mut self) -> &mut HashMap<GeometryId, Lines> {
        &mut self.lines
    }
    pub fn get_surfaces_mut(&mut self) -> &mut HashMap<GeometryId, Surface> {
        &mut self.surfaces
    }
    pub fn get_polylines_mut(&mut self) -> &mut HashMap<GeometryId, Polyline> {
        &mut self.polylines
    }
    pub fn get_meshes_mut(&mut self) -> &mut HashMap<GeometryId, Mesh> {
        &mut self.meshes
    }
    pub fn get_curves(&self) -> &HashMap<GeometryId, Curve> {
        &self.curves
    }
    pub fn get_lines(&self) -> &HashMap<GeometryId, Lines> {
        &self.lines
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
    pub fn add_curve(&mut self, curve: Curve) -> GeometryId {
        let id = new_geometry_id();
        self.curves.insert(id, curve);
        id
    }
    pub fn add_lines(&mut self, lines: Lines) -> GeometryId {
        let id = new_geometry_id();
        self.lines.insert(id, lines);
        id
    }
    pub fn add_surface(&mut self, surface: Surface) -> GeometryId {
        let id = new_geometry_id();
        self.surfaces.insert(id, surface);
        id
    }
    pub fn add_polyline(&mut self, polyline: Polyline) -> GeometryId {
        let id = new_geometry_id();
        self.polylines.insert(id, polyline);
        id
    }
    pub fn add_mesh(&mut self, mesh: Mesh) -> GeometryId {
        let id = new_geometry_id();
        self.meshes.insert(id, mesh);
        id
    }
    pub fn get_geometry(&mut self, geometry_id: GeometryId) -> Option<&mut dyn Geometry> {
        // TODO: Mathch
        if let Some(curve) = self.curves.get_mut(&geometry_id) {
            Some(curve)
        } else if let Some(mesh) = self.meshes.get_mut(&geometry_id) {
            Some(mesh)
        } else if let Some(surface) = self.surfaces.get_mut(&geometry_id) {
            Some(surface)
        } else if let Some(polyline) = self.polylines.get_mut(&geometry_id) {
            Some(polyline)
        } else if let Some(lines) = self.lines.get_mut(&geometry_id) {
            Some(lines)
        } else {
            None
        }
    }
    pub fn delete_geometry(&mut self, geoemtry_id: GeometryId) {
        self.curves.remove(&geoemtry_id);
        self.meshes.remove(&geoemtry_id);
        self.polylines.remove(&geoemtry_id);
        self.surfaces.remove(&geoemtry_id);
        self.lines.remove(&geoemtry_id);
    }

    pub fn rotate_geometry(&mut self, id: GeometryId, center: &[f32], axis: &[f32], radians: f32) {
        if let Some(geo) = self.get_geometry(id) {
            geo.rotate(center.into(), axis.into(), radians);
        }
    }
}
