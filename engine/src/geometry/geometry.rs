use std::sync::Mutex;

use crate::math::linear_algebra::{mat4::Mat4, vec3::Vec3, vec4::Vec4};

use super::bind_group::GeometryBindGroupObject;

pub type GeometryId = u64;

static mut GEOMETRY_ID_GENERATOR: Mutex<GeometryId> = Mutex::new(0u64);

pub fn new_geometry_id() -> GeometryId {
    unsafe {
        let mut changer = GEOMETRY_ID_GENERATOR.lock().unwrap();
        *changer += 1u64;
        *changer
    }
}

// TODO: time for polymorphism
pub trait Geometry {
    fn get_bind_group_object_mut(&mut self) -> &mut GeometryBindGroupObject;

    fn rotate(&mut self, center: Vec3, axis: Vec3, radians: f32) {
        self.get_bind_group_object_mut()
            .rotate(center, axis, radians);
    }
}
