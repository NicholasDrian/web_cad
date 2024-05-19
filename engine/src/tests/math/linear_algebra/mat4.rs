use crate::math::linear_algebra::mat4::*;
use crate::math::linear_algebra::vec4::*;

use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn test_mul() {
    let m_i = Mat4::identity();
    let m_a = Mat4::new(&[
        0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0,
    ]);
    let m_b = Mat4::new(&[
        10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0,
        25.0,
    ]);
    let m_ab = Mat4::new(&[
        296.0, 342.0, 388.0, 434.0, 392.0, 454.0, 516.0, 578.0, 488.0, 566.0, 644.0, 722.0, 584.0,
        678.0, 772.0, 866.0,
    ]);
    assert_eq!(Mat4::multiply(&m_i, &m_a), m_a);
    assert_eq!(Mat4::multiply(&m_a, &m_i), m_a);
    assert_eq!(Mat4::multiply(&m_a, &m_b), m_ab);
}

#[wasm_bindgen_test]
fn test_transform() {
    let m_i = Mat4::identity();
    let m_t = Mat4::new(&[
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
    ]);
    let v = Vec4 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
        w: 4.0,
    };
    let tv = Vec4 {
        x: 90.0,
        y: 100.0,
        z: 110.0,
        w: 120.0,
    };
    assert_eq!(m_i.transform(&v), v);
    assert_eq!(m_t.transform(&v), tv);
}

#[wasm_bindgen_test]
fn test_rotate_point_axis() {
    assert!(false);
}

#[wasm_bindgen_test]
fn translate() {
    assert!(false);
}
