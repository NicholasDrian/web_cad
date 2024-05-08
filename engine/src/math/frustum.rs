use super::{bounding_box::BoundingBox, mat4::Mat4, ray::Ray, vec3::Vec3};

#[derive(Debug, Copy, Clone)]
pub struct Frustum {
    origin: Vec3,
    up: Vec3,
    right: Vec3,
    down: Vec3,
    left: Vec3,
    top_left: Vec3,
    top_right: Vec3,
    bottom_left: Vec3,
    bottom_right: Vec3,
}

impl Frustum {
    pub fn new(top_left: &Ray, top_right: &Ray, bottom_right: &Ray, bottom_left: &Ray) -> Frustum {
        let up = Vec3::cross(top_left.get_direction(), top_right.get_direction()).to_normalized();
        let right =
            Vec3::cross(top_right.get_direction(), bottom_right.get_direction()).to_normalized();
        let down =
            Vec3::cross(bottom_right.get_direction(), bottom_left.get_direction()).to_normalized();
        let left =
            Vec3::cross(bottom_left.get_direction(), top_left.get_direction()).to_normalized();
        Frustum {
            origin: *top_left.get_origin(),
            up,
            right,
            down,
            left,
            top_left: *top_left.get_direction(),
            top_right: *top_right.get_direction(),
            bottom_left: *bottom_left.get_direction(),
            bottom_right: *bottom_right.get_direction(),
        }
    }
    pub fn contains_point(&self, point: &Vec3) -> bool {
        let v = Vec3::subtract(point, &self.origin);
        Vec3::dot(&self.up, &v) > 0.0
            || Vec3::dot(&self.right, &v) > 0.0
            || Vec3::dot(&self.down, &v) > 0.0
            || Vec3::dot(&self.left, &v) > 0.0
    }

    pub fn contains_line_fully(&self, start: &Vec3, end: &Vec3) -> bool {
        self.contains_point(start) && self.contains_point(end)
    }

    pub fn contains_line_partially(&self, start: &Vec3, end: &Vec3) -> bool {
        let mut dir = Vec3::subtract(end, start);
        let len = dir.len();
        dir.scale(1.0 / len);

        let ray = Ray::new(*start, dir);

        todo!();

        true
    }

    pub fn contains_bounding_box_fully(bb: BoundingBox) -> bool {
        todo!();
    }
    pub fn contains_bounding_box_partially(bb: BoundingBox) -> bool {
        todo!();
    }

    pub fn transform(&mut self, t: Mat4) -> &mut Self {
        self.origin = t.transform_point(&self.origin);
        self.top_left = t.transform_vector(&self.top_left);
        self.top_right = t.transform_vector(&self.top_right);
        self.bottom_right = t.transform_vector(&self.bottom_right);
        self.bottom_left = t.transform_vector(&self.bottom_left);

        let forward = Vec3::add(
            &self.top_left,
            &Vec3::add(
                &self.top_right,
                &Vec3::add(&self.bottom_left, &self.bottom_right),
            ),
        );

        self.up = Vec3::cross(&self.top_left, &self.top_right).to_normalized();
        self.right = Vec3::cross(&self.top_right, &self.bottom_right).to_normalized();
        self.down = Vec3::cross(&self.bottom_right, &self.bottom_left).to_normalized();
        self.left = Vec3::cross(&self.bottom_left, &self.top_left).to_normalized();

        if Vec3::dot(&self.up, &forward) < 0.0 {
            self.up.scale(-1.0);
            self.down.scale(-1.0);
        }

        if Vec3::dot(&self.left, &forward) < 0.0 {
            self.left.scale(-1.0);
            self.right.scale(-1.0);
        }

        self
    }
}
