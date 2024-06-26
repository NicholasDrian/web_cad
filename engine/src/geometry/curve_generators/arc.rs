use crate::{
    geometry::{curve::Curve, GeometryId},
    gpu_samplers::curve_sampler::CurveSampler,
    math::{
        geometry::{plane::Plane, ray::Ray},
        linear_algebra::{vec3::Vec3, vec4::Vec4},
    },
    scene::scene_interface::Scene,
    utils::get_instance_mut,
};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl Scene {
    #[wasm_bindgen]
    pub fn add_arc_start_middle_end(
        &self,
        start: &[f32],
        middle: &[f32],
        end: &[f32],
    ) -> GeometryId {
        let start: Vec3 = start.into();
        let middle: Vec3 = middle.into();
        let end: Vec3 = end.into();

        let ab = Vec3::subtract(&start, &middle);
        let ac = Vec3::subtract(&end, &middle);
        let normal = Vec3::to_normalized(&Vec3::cross(&ab, &ac));

        let ro = Vec3::to_scaled(&Vec3::add(&middle, &start), 0.5);
        let rd = Vec3::to_normalized(&Vec3::cross(&ab, &normal));
        let r = Ray::new(ro, rd);

        let po = Vec3::to_scaled(&Vec3::add(&middle, &end), 0.5);
        let pn = Vec3::to_normalized(&ac);
        let p = Plane::new(po, pn);

        let t = r.intersect_plane(&p, true).unwrap();
        let center = r.at(t);
        let radius = Vec3::distance(&middle, &center);

        let x_axis = Vec3::to_normalized(&Vec3::subtract(&end, &center));
        let y_axis = Vec3::cross(&normal, &x_axis);

        let mut theta = Vec3::angle_between(
            &Vec3::subtract(&start, &center),
            &Vec3::subtract(&end, &center),
        );

        if Vec3::angle_between(&ab, &ac) < std::f32::consts::PI / 2.0 {
            theta = 2.0 * std::f32::consts::PI - theta;
        }

        if f32::is_nan(theta) || theta == 0.0 {
            log::info!("create arc failed");
            return 0;
        }

        let curve = create_arc(
            get_instance_mut!(&self.get_instance_handle()).get_curve_sampler(),
            center,
            x_axis,
            y_axis,
            radius,
            0.0,
            theta,
        );

        get_instance_mut!(&self.get_instance_handle())
            .get_scene_mut(self.get_handle())
            .add_curve(curve)
    }

    #[wasm_bindgen]
    pub fn create_arc_center_start_end(
        center: &[f32],
        start: &[f32],
        end: &[f32],
        over_180_degrees: bool,
    ) -> GeometryId {
        todo!()
    }
}

pub fn create_arc(
    sampler: &CurveSampler,
    origin: Vec3,
    x_axis: Vec3,
    y_axis: Vec3,
    radius: f32,
    theta_start: f32,
    mut theta_end: f32,
) -> Curve {
    if theta_end < theta_start {
        theta_end += 360.0;
    }

    let theta = theta_end - theta_start;

    let arc_count = if theta <= std::f32::consts::PI / 2.0 {
        1
    } else if theta <= std::f32::consts::PI {
        2
    } else if theta <= 3.0 * std::f32::consts::PI / 2.0 {
        3
    } else {
        4
    };

    let d_theta: f32 = theta / arc_count as f32;

    let weight = f32::cos(d_theta / 2.0);

    let mut point_0: Vec3 = Vec3::add(
        &origin,
        &Vec3::add(
            &Vec3::to_scaled(&x_axis, radius * f32::cos(theta_start)),
            &Vec3::to_scaled(&y_axis, radius * f32::sin(theta_start)),
        ),
    );

    let mut tangent_0: Vec3 = Vec3::add(
        &Vec3::to_scaled(&x_axis, f32::sin(theta_start)),
        &Vec3::to_scaled(&y_axis, f32::cos(theta_start)),
    );

    let mut weighted_controls: Vec<Vec4> = vec![Vec4 {
        x: point_0.x,
        y: point_0.y,
        z: point_0.z,
        w: 1.0,
    }];

    let mut angle: f32 = theta_start;

    for i in 1..=arc_count {
        angle += d_theta;

        let point_2: Vec3 = Vec3::add(
            &origin,
            &Vec3::add(
                &Vec3::to_scaled(&x_axis, radius * f32::cos(angle)),
                &Vec3::to_scaled(&y_axis, radius * f32::sin(angle)),
            ),
        );

        let tangent_2: Vec3 = Vec3::add(
            &Vec3::to_scaled(&x_axis, -f32::sin(angle)),
            &Vec3::to_scaled(&y_axis, f32::cos(angle)),
        );

        let ray_0 = Ray::new(point_0, tangent_0);
        let ray_1 = Ray::new(point_2, tangent_2);
        let point_1: Vec3 = ray_0.closest_point_to_line(
            ray_1.get_origin(),
            &Vec3::add(ray_1.get_direction(), ray_1.get_origin()),
        );

        weighted_controls.push(Vec3::to_scaled(&point_1, weight).append(weight));
        weighted_controls.push(point_2.append(1.0));

        if i < arc_count {
            point_0 = point_2;
            tangent_0 = tangent_2;
        }
    }

    let knots: Vec<f32> = match arc_count {
        1 => vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
        2 => vec![0.0, 0.0, 0.0, 1.0 / 2.0, 1.0 / 2.0, 1.0, 1.0, 1.0],
        3 => vec![
            0.0,
            0.0,
            0.0,
            1.0 / 3.0,
            1.0 / 3.0,
            2.0 / 3.0,
            2.0 / 3.0,
            1.0,
            1.0,
            1.0,
        ],
        4 => vec![
            0.0,
            0.0,
            0.0,
            1.0 / 4.0,
            1.0 / 4.0,
            2.0 / 4.0,
            2.0 / 4.0,
            3.0 / 4.0,
            3.0 / 4.0,
            1.0,
            1.0,
            1.0,
        ],
        _ => unreachable!(),
    };

    Curve::new(sampler, 2, weighted_controls, &knots[..])
}
