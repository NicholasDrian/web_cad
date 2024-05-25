//! NOTE: Creating a bb for a mesh alters its index buffer.
use crate::math::geometry::ray::Ray;

use crate::gpu_ray_tracing::intersection::Intersection;

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
