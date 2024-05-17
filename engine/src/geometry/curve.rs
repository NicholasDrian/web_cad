use crate::{
    math::linear_algebra::vec3::Vec3,
    math::linear_algebra::vec4::Vec4,
    samplers::{curve_sampler::CurveSampler, params::SAMPLES_PER_SEGMENT},
};

use super::{geometry::Geometry, utils::default_knot_vector};

pub struct Curve {
    degree: u32,
    controls: Vec<Vec3>,
    weights: Vec<f32>,
    knots: Vec<f32>,
    // Samples
    vertex_buffer: wgpu::Buffer,
    vertex_count: u32,
}

impl Curve {
    pub fn new(
        curve_sampler: &CurveSampler,
        degree: u32,
        controls: Vec<Vec3>,
        weights: &[f32],
        knots: &[f32],
    ) -> Curve {
        let knots = if knots.len() == 0 {
            default_knot_vector(controls.len(), degree)
        } else {
            knots.to_vec()
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
        let vertex_buffer = curve_sampler.sample_curve(degree, &weighted_controls, &knots);
        let vertex_count = SAMPLES_PER_SEGMENT * (controls.len() as u32 - 1) + 1;
        Curve {
            degree,
            controls,
            weights,
            knots,
            vertex_buffer,
            vertex_count,
        }
    }

    pub fn get_vertex_buffer(&self) -> &wgpu::Buffer {
        &self.vertex_buffer
    }

    pub fn get_vertex_count(&self) -> u32 {
        self.vertex_count
    }
}

impl Geometry for Curve {
    fn rotate_about_z(&mut self, radians: f32) {
        todo!()
    }
}
