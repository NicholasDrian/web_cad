use std::rc::Rc;

use crate::{
    math::linear_algebra::vec3::Vec3,
    render::renderer::Renderer,
    samplers::{curve_sampler::CurveSampler, params::SAMPLES_PER_SEGMENT},
};

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
        weights: Vec<f32>,
        knots: Vec<f32>,
    ) -> Curve {
        let vertex_buffer = curve_sampler.sample_curve(degree, &controls, &weights, &knots);
        let vertex_count = SAMPLES_PER_SEGMENT * (controls.len() as u32 - 1);
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
