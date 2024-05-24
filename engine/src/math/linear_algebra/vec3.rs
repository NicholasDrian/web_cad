use super::vec4::Vec4;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[wasm_bindgen]
pub fn new_vec_3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3 { x, y, z }
}

impl From<&[f32]> for Vec3 {
    fn from(value: &[f32]) -> Self {
        Self {
            x: value[0],
            y: value[1],
            z: value[2],
        }
    }
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    pub fn dot(a: &Vec3, b: &Vec3) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn cross(a: &Vec3, b: &Vec3) -> Vec3 {
        Vec3 {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x,
        }
    }

    pub fn subtract(a: &Vec3, b: &Vec3) -> Vec3 {
        Vec3 {
            x: a.x - b.x,
            y: a.y - b.y,
            z: a.z - b.z,
        }
    }

    pub fn add(a: &Vec3, b: &Vec3) -> Vec3 {
        Vec3 {
            x: a.x + b.x,
            y: a.y + b.y,
            z: a.z + b.z,
        }
    }

    pub fn append(&self, n: f32) -> Vec4 {
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w: n,
        }
    }
    pub fn angle_between(a: &Vec3, b: &Vec3) -> f32 {
        todo!()
    }

    pub fn distance(a: &Vec3, b: &Vec3) -> f32 {
        let delta = Vec3::subtract(a, b);
        Vec3::dot(&delta, &delta)
    }

    pub fn rotate(v: &Vec3, center: &Vec3, axis: &Vec3, theta: f32) -> Vec3 {
        todo!();
    }

    pub fn normalize(&mut self) -> &mut Self {
        let size_square = self.x * self.x + self.y * self.y + self.z * self.z;
        if size_square == 0.0 {
            self.x = 1.0;
            self.y = 0.0;
            self.z = 0.0;
            log::warn!("sketch!");
            return self;
        }
        let size = size_square.sqrt();
        self.x /= size;
        self.y /= size;
        self.z /= size;
        self
    }

    pub fn to_normalized(&self) -> Vec3 {
        let size_square = self.x * self.x + self.y * self.y + self.z * self.z;
        if size_square == 0.0 {
            log::warn!("sketch?");
            return Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            };
        }
        let size = size_square.sqrt();
        Vec3 {
            x: self.x / size,
            y: self.y / size,
            z: self.z / size,
        }
    }

    pub fn len(&self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn scale(&mut self, s: f32) {
        self.x *= s;
        self.y *= s;
        self.z *= s;
    }

    pub fn to_scaled(v: &Vec3, s: f32) -> Vec3 {
        Vec3 {
            x: v.x * s,
            y: v.y * s,
            z: v.z * s,
        }
    }
}
