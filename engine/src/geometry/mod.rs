pub mod bind_group;
pub mod curve;
pub mod curve_generators;
pub mod lines;
pub mod mesh;
pub mod polyline;
pub mod surface;
pub mod surface_generators;
pub mod utils;

use std::sync::Mutex;

use crate::math::linear_algebra::vec3::Vec3;

use bind_group::GeometryBindGroupObject;

pub type GeometryId = u32;

static mut GEOMETRY_ID_GENERATOR: Mutex<GeometryId> = Mutex::new(0u32);

pub fn new_geometry_id() -> GeometryId {
    unsafe {
        let mut changer = GEOMETRY_ID_GENERATOR.lock().unwrap();
        *changer += 1u32;
        *changer
    }
}

pub trait Geometry {
    fn get_bind_group_object_mut(&mut self) -> &mut GeometryBindGroupObject;

    fn rotate(&mut self, center: Vec3, axis: Vec3, radians: f32) {
        self.get_bind_group_object_mut()
            .rotate(center, axis, radians);
    }
}
