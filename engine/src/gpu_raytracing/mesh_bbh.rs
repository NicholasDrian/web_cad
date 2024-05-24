use crate::math::geometry::ray::Ray;

use super::intersection::Intersection;

pub struct MeshBBH {
    buffer: wgpu::Buffer,
}

impl MeshBBH {
    pub fn new() -> Self {
        todo!()
    }

    pub fn intersect(ray: Ray) -> Option<Intersection> {
        todo!();
    }
}
