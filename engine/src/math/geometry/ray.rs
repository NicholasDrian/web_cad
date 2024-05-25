use crate::math::linear_algebra::vec3::Vec3;

use super::plane::Plane;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            origin,
            direction: direction.to_normalized(),
        }
    }
    pub fn at(&self, t: f32) -> Vec3 {
        Vec3::add(&self.origin, &Vec3::to_scaled(&self.direction, t))
    }
    pub fn get_origin(&self) -> &Vec3 {
        &self.origin
    }
    pub fn get_direction(&self) -> &Vec3 {
        &self.direction
    }

    /// Could be optimized by making multiple functions to reduce branch count
    pub fn intersect_plane(&self, plane: &Plane, allow_negative: bool) -> Option<f32> {
        let numerator = Vec3::dot(
            &Vec3::subtract(plane.get_origin(), &self.origin),
            plane.get_normal(),
        );
        let denominator = Vec3::dot(&self.direction, plane.get_normal());
        if denominator == 0.0 {
            return None;
        }
        let t = numerator / denominator;
        if allow_negative {
            return Some(t);
        }
        if t < 0.0 {
            return None;
        }
        Some(t)
    }

    pub fn closest_point_to_line(&self, start: &Vec3, end: &Vec3) -> Vec3 {
        let p = Vec3::add(&self.origin, &self.direction);

        let v13 = Vec3::subtract(start, &self.origin);
        let v43 = Vec3::subtract(&p, &self.origin);
        let v21 = Vec3::subtract(end, start);

        let d1343 = Vec3::dot(&v13, &v43);
        let d4321 = Vec3::dot(&v43, &v21);
        let d1321 = Vec3::dot(&v13, &v21);
        let d4343 = Vec3::dot(&v43, &v43);
        let d2121 = Vec3::dot(&v21, &v21);

        let mua = (d1343 * d4321 - d1321 * d4343) / (d2121 * d4343 - d4321 * d4321);
        let mub = (d1343 + mua * d4321) / d4343;

        return Vec3::add(&self.origin, &Vec3::to_scaled(&v43, mub));
    }
}
