use crate::math::linear_algebra::{mat3::Mat3, vec3::Vec3};

use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn test_mul() {
    let m_a = Mat3 {
        nums: [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0],
    };
    let m_b = Mat3 {
        nums: [2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0, 18.0],
    };
    let m_i = Mat3::identity();

    let m_ab = Mat3 {
        nums: [60.0, 72.0, 84.0, 132.0, 162.0, 192.0, 204.0, 252.0, 300.0],
    };

    assert_eq!(Mat3::multiply(&m_a, &m_b), m_ab);
    assert_eq!(Mat3::multiply(&m_a, &m_i), m_a);
    assert_eq!(Mat3::multiply(&m_i, &m_a), m_a);
}
#[wasm_bindgen_test]
fn test_transform() {
    let m_t = Mat3 {
        nums: [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0],
    };
    let m_i = Mat3::identity();
    let m_v = Vec3 {
        x: 3.0,
        y: 7.0,
        z: 1.0,
    };
    let m_tv = Vec3 {
        x: 3.0 * 1.0 + 7.0 * 4.0 + 1.0 * 7.0,
        y: 3.0 * 2.0 + 7.0 * 5.0 + 1.0 * 8.0,
        z: 3.0 * 3.0 + 7.0 * 6.0 + 1.0 * 9.0,
    };
    assert_eq!(m_t.transform(&m_v), m_tv);
    assert_eq!(m_i.transform(&m_v), m_v);
}
