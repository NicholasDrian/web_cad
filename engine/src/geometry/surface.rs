use crate::{
    math::linear_algebra::{vec3::Vec3, vec4::Vec4},
    samplers::{params::SAMPLES_PER_SEGMENT, surface_sampler::SurfaceSampler},
};

use super::{geometry::Geometry, utils::default_knot_vector};

pub struct Surface {
    controls: Vec<Vec3>,
    control_count_u: u32,
    control_count_v: u32,
    degree_u: u32,
    degree_v: u32,
    /// Leave empty for default values
    weights: Vec<f32>,
    /// Leave empty for default values
    knots_u: Vec<f32>,
    /// Leave empty for default values
    knots_v: Vec<f32>,
    vertex_buffer: wgpu::Buffer,
    vertex_count: u32,
    index_buffer: wgpu::Buffer,
}

impl Surface {
    pub fn new(
        surface_sampler: &SurfaceSampler,
        control_count_u: u32,
        control_count_v: u32,
        degree_u: u32,
        degree_v: u32,
        controls: Vec<Vec3>,
        weights: &[f32],
        knots_u: &[f32],
        knots_v: &[f32],
    ) -> Self {
        let knots_u = if knots_u.len() == 0 {
            default_knot_vector(control_count_u as usize, degree_u)
        } else {
            knots_u.to_vec()
        };
        let weights = if weights.len() == 0 {
            vec![1.0; controls.len()]
        } else {
            weights.to_vec()
        };
        let weighted_controls: Vec<Vec4> = controls
            .iter()
            .zip(weights.iter())
            .map(|(control, weight)| Vec4 {
                x: control.x * weight,
                y: control.y * weight,
                z: control.z * weight,
                w: *weight,
            })
            .collect();
        let vertex_buffer = surface_sampler.sample_surface(
            degree_u,
            degree_v,
            &weighted_controls[..],
            control_count_u,
            control_count_v,
            &knots_u[..],
            &knots_v[..],
        );
        let vertex_count = SAMPLES_PER_SEGMENT
            * (control_count_u - 1)
            * SAMPLES_PER_SEGMENT
            * (control_count_v - 1);

        let index_buffer = todo!();

        Self {
            controls: todo!(),
            control_count_u: todo!(),
            control_count_v: todo!(),
            degree_u: todo!(),
            degree_v: todo!(),
            weights: todo!(),
            knots_u: todo!(),
            knots_v: todo!(),
            vertex_buffer,
            vertex_count,
            index_buffer,
        }
    }
}

impl Geometry for Surface {}
