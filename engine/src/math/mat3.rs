use crate::math::vec3::Vec3;

/// column major 3x3 matrix.
#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct Mat3 {
    pub nums: [f32; 9],
}

impl Mat3 {
    pub fn identity() -> Mat3 {
        Mat3 {
            nums: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
        }
    }

    pub fn multiply(a: &Mat3, b: &Mat3) -> Mat3 {
        let a00 = a.nums[0];
        let a01 = a.nums[1];
        let a02 = a.nums[2];

        let a10 = a.nums[3];
        let a11 = a.nums[4];
        let a12 = a.nums[5];

        let a20 = a.nums[6];
        let a21 = a.nums[7];
        let a22 = a.nums[8];

        let b00 = b.nums[0];
        let b01 = b.nums[1];
        let b02 = b.nums[2];

        let b10 = b.nums[3];
        let b11 = b.nums[4];
        let b12 = b.nums[5];

        let b20 = b.nums[6];
        let b21 = b.nums[7];
        let b22 = b.nums[8];

        Mat3 {
            nums: [
                a00 * b00 + a10 * b01 + a20 * b02,
                a01 * b00 + a11 * b01 + a21 * b02,
                a02 * b00 + a12 * b01 + a22 * b02,
                a00 * b10 + a10 * b11 + a20 * b12,
                a01 * b10 + a11 * b11 + a21 * b12,
                a02 * b10 + a12 * b11 + a22 * b12,
                a00 * b20 + a10 * b21 + a20 * b22,
                a01 * b20 + a11 * b21 + a21 * b22,
                a02 * b20 + a12 * b21 + a22 * b22,
            ],
        }
    }

    pub fn transform(self, v: &Vec3) -> Vec3 {
        let a00 = self.nums[0];
        let a01 = self.nums[1];
        let a02 = self.nums[2];

        let a10 = self.nums[3];
        let a11 = self.nums[4];
        let a12 = self.nums[5];

        let a20 = self.nums[6];
        let a21 = self.nums[7];
        let a22 = self.nums[8];

        Vec3 {
            x: v.x * a00 + v.y * a10 + v.z * a20,
            y: v.x * a01 + v.y * a11 + v.z * a21,
            z: v.x * a02 + v.y * a12 + v.z * a22,
        }
    }
}
