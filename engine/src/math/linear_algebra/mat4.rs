use super::{vec3::Vec3, vec4::Vec4};

/// column major 4x4 matrix.
///
/// |a e i m|
/// |b f j n|
/// |c g k o|
/// |d h l p|
///
#[repr(C)]
#[derive(Debug, PartialEq, Copy, Clone, Default, bytemuck::Zeroable, bytemuck::Pod)]
pub struct Mat4 {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub e: f32,
    pub f: f32,
    pub g: f32,
    pub h: f32,
    pub i: f32,
    pub j: f32,
    pub k: f32,
    pub l: f32,
    pub m: f32,
    pub n: f32,
    pub o: f32,
    pub p: f32,
}

impl Mat4 {
    pub fn new(nums: &[f32; 16]) -> Mat4 {
        Mat4 {
            a: nums[0],
            b: nums[1],
            c: nums[2],
            d: nums[3],
            e: nums[4],
            f: nums[5],
            g: nums[6],
            h: nums[7],
            i: nums[8],
            j: nums[9],
            k: nums[10],
            l: nums[11],
            m: nums[12],
            n: nums[13],
            o: nums[14],
            p: nums[15],
        }
    }

    pub fn identity() -> Mat4 {
        Mat4 {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 0.0,
            e: 0.0,
            f: 1.0,
            g: 0.0,
            h: 0.0,
            i: 0.0,
            j: 0.0,
            k: 1.0,
            l: 0.0,
            m: 0.0,
            n: 0.0,
            o: 0.0,
            p: 1.0,
        }
    }

    pub fn look_at(position: &Vec3, focal_point: &Vec3, up: &Vec3) -> Mat4 {
        let z_axis = Vec3::subtract(position, focal_point).to_normalized();
        let x_axis = Vec3::cross(up, &z_axis).to_normalized();
        let y_axis = Vec3::cross(&z_axis, &x_axis);
        Mat4 {
            a: x_axis.x,
            b: y_axis.x,
            c: z_axis.x,
            d: 0.0,
            e: x_axis.y,
            f: y_axis.y,
            g: z_axis.y,
            h: 0.0,
            i: x_axis.z,
            j: y_axis.z,
            k: z_axis.z,
            l: 0.0,
            m: -Vec3::dot(&x_axis, position),
            n: -Vec3::dot(&y_axis, position),
            o: -Vec3::dot(&z_axis, position),
            p: 1.0,
        }
    }

    /// z_far can be infinity
    pub fn perspective(fovy: f32, aspect: f32, near_dist: f32, far_dist: f32) -> Mat4 {
        let temp = f32::tan((std::f32::consts::PI - fovy) / 2.0);
        Mat4 {
            a: temp / aspect,
            b: 0.0,
            c: 0.0,
            d: 0.0,
            e: 0.0,
            f: temp,
            g: 0.0,
            h: 0.0,
            i: 0.0,
            j: 0.0,
            k: if far_dist == f32::INFINITY {
                -1.0
            } else {
                far_dist / (near_dist - far_dist)
            },
            l: -1.0,
            m: 0.0,
            n: 0.0,
            o: if far_dist == f32::INFINITY {
                -near_dist
            } else {
                far_dist * near_dist / (near_dist - far_dist)
            },
            p: 0.0,
        }
    }

    pub fn translation(t: &Vec3) -> Mat4 {
        Mat4 {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 0.0,
            e: 0.0,
            f: 1.0,
            g: 0.0,
            h: 0.0,
            i: 0.0,
            j: 0.0,
            k: 1.0,
            l: 0.0,
            m: t.x,
            n: t.y,
            o: t.z,
            p: 1.0,
        }
    }

    pub fn rotation(axis: &Vec3, theta: f32) -> Mat4 {
        let normalized_axis = axis.to_normalized();
        let xx = normalized_axis.x * normalized_axis.x;
        let yy = normalized_axis.y * normalized_axis.y;
        let zz = normalized_axis.z * normalized_axis.z;
        let cos = f32::cos(theta);
        let sin = f32::sin(theta);
        let one_minus_cosine = 1.0 - cos;
        Mat4 {
            a: xx + (1.0 - xx) * cos,
            b: normalized_axis.x * normalized_axis.y * one_minus_cosine + normalized_axis.z * sin,
            c: normalized_axis.x * normalized_axis.z * one_minus_cosine - normalized_axis.y * sin,
            d: 0.0,
            e: normalized_axis.x * normalized_axis.y * one_minus_cosine - normalized_axis.z * sin,
            f: yy + (1.0 - yy) * cos,
            g: normalized_axis.y * normalized_axis.z * one_minus_cosine + normalized_axis.x * sin,
            h: 0.0,
            i: normalized_axis.x * normalized_axis.z * one_minus_cosine + normalized_axis.y * sin,
            j: normalized_axis.y * normalized_axis.z * one_minus_cosine - normalized_axis.x * sin,
            k: zz + (1.0 - zz) * cos,
            l: 0.0,
            m: 0.0,
            n: 0.0,
            o: 0.0,
            p: 1.0,
        }
    }

    pub fn multiply(a: &Mat4, b: &Mat4) -> Mat4 {
        Mat4 {
            a: a.a * b.a + a.e * b.b + a.i * b.c + a.m * b.d,
            b: a.b * b.a + a.f * b.b + a.j * b.c + a.n * b.d,
            c: a.c * b.a + a.g * b.b + a.k * b.c + a.o * b.d,
            d: a.d * b.a + a.h * b.b + a.l * b.c + a.p * b.d,
            e: a.a * b.e + a.e * b.f + a.i * b.g + a.m * b.h,
            f: a.b * b.e + a.f * b.f + a.j * b.g + a.n * b.h,
            g: a.c * b.e + a.g * b.f + a.k * b.g + a.o * b.h,
            h: a.d * b.e + a.h * b.f + a.l * b.g + a.p * b.h,
            i: a.a * b.i + a.e * b.j + a.i * b.k + a.m * b.l,
            j: a.b * b.i + a.f * b.j + a.j * b.k + a.n * b.l,
            k: a.c * b.i + a.g * b.j + a.k * b.k + a.o * b.l,
            l: a.d * b.i + a.h * b.j + a.l * b.k + a.p * b.l,
            m: a.a * b.m + a.e * b.n + a.i * b.o + a.m * b.p,
            n: a.b * b.m + a.f * b.n + a.j * b.o + a.n * b.p,
            o: a.c * b.m + a.g * b.n + a.k * b.o + a.o * b.p,
            p: a.d * b.m + a.h * b.n + a.l * b.o + a.p * b.p,
        }
    }

    pub fn transform(self, v: &Vec4) -> Vec4 {
        let v00 = self.a;
        let v01 = self.e;
        let v02 = self.i;
        let v03 = self.m;
        let v10 = self.b;
        let v11 = self.f;
        let v12 = self.j;
        let v13 = self.n;
        let v20 = self.c;
        let v21 = self.g;
        let v22 = self.k;
        let v23 = self.o;
        let v30 = self.d;
        let v31 = self.h;
        let v32 = self.l;
        let v33 = self.p;

        Vec4 {
            x: v.x * v00 + v.y * v01 + v.z * v02 + v.w * v03,
            y: v.x * v10 + v.y * v11 + v.z * v12 + v.w * v13,
            z: v.x * v20 + v.y * v21 + v.z * v22 + v.w * v23,
            w: v.x * v30 + v.y * v31 + v.z * v32 + v.w * v33,
        }
    }

    pub fn transform_point(&self, p: &Vec3) -> Vec3 {
        let temp = self.transform(&Vec4 {
            x: p.x,
            y: p.y,
            z: p.z,
            w: 1.0,
        });
        Vec3 {
            x: temp.x / temp.w,
            y: temp.y / temp.w,
            z: temp.z / temp.w,
        }
    }

    pub fn transform_vector(&self, v: &Vec3) -> Vec3 {
        let temp = self.transform(&Vec4 {
            x: v.x,
            y: v.y,
            z: v.z,
            w: 0.0,
        });
        Vec3 {
            x: temp.x / temp.w,
            y: temp.y / temp.w,
            z: temp.z / temp.w,
        }
    }
}
