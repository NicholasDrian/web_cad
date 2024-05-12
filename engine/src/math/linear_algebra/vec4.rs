use crate::math::linear_algebra::vec3::Vec3;

#[repr(C)]
#[derive(Debug, Copy, Clone, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub fn dot(a: &Vec4, b: &Vec4) -> Vec4 {
        Vec4 {
            x: a.x * b.x,
            y: a.y * b.y,
            z: a.z * b.z,
            w: a.w * b.w,
        }
    }

    pub fn to_vec3_safe(self) -> Vec3 {
        if self.w == 0.0 {
            log::warn!("sketch");
            return Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
        }
        Vec3 {
            x: self.x / self.w,
            y: self.y / self.w,
            z: self.z / self.w,
        }
    }
}
