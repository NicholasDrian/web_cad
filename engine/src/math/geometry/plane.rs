use crate::math::linear_algebra::vec3::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Plane {
    origin: Vec3,
    normal: Vec3,
}

impl Plane {
    pub fn new(origin: Vec3, normal: Vec3) -> Self {
        Self { origin, normal }
    }
    pub fn get_origin(&self) -> &Vec3 {
        &self.origin
    }
    pub fn get_normal(&self) -> &Vec3 {
        &self.normal
    }
}
