use crate::math::linear_algebra::vec3::Vec3;

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
    pub fn get_origin(&self) -> &Vec3 {
        &self.origin
    }
    pub fn get_direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn intersect_plane(plane_origin: &Vec3, plane_normal: &Vec3) -> Option<f32> {
        todo!();
    }

    pub fn closest_point_to_line(&self, start: &Vec3, end: &Vec3) -> Vec3 {
        let p = Vec3::add(&self.origin, &self.direction);

        let v13 = Vec3::subtract(&start, &self.origin);
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
