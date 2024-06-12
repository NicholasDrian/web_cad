use crate::{
    gpu_samplers::{curve_sampler::CurveSampler, params::SAMPLES_PER_SEGMENT},
    math::linear_algebra::vec4::Vec4,
};

use super::{bind_group::GeometryBindGroupObject, utils::default_knot_vector, Geometry};

pub struct Curve {
    degree: u32,
    weighted_controls: Vec<Vec4>,
    bind_group_object: GeometryBindGroupObject,
    knots: Vec<f32>,
    // Samples
    vertex_buffer: wgpu::Buffer,
    vertex_count: u32,
}

impl Curve {
    pub fn new(
        curve_sampler: &CurveSampler,
        degree: u32,
        weighted_controls: Vec<Vec4>,
        knots: &[f32],
    ) -> Curve {
        let knots = if knots.len() == 0 {
            default_knot_vector(weighted_controls.len(), degree)
        } else {
            knots.to_vec()
        };

        let vertex_buffer = curve_sampler.sample_curve(degree, &weighted_controls, &knots);
        let vertex_count = SAMPLES_PER_SEGMENT * (weighted_controls.len() as u32 - 1) + 1;
        let bind_group_object = GeometryBindGroupObject::new(curve_sampler.get_renderer());
        Curve {
            degree,
            weighted_controls,
            knots,
            vertex_buffer,
            vertex_count,
            bind_group_object,
        }
    }

    pub fn get_vertex_buffer(&self) -> &wgpu::Buffer {
        &self.vertex_buffer
    }

    pub fn get_vertex_count(&self) -> u32 {
        self.vertex_count
    }

    pub fn get_bind_group(&self) -> &wgpu::BindGroup {
        self.bind_group_object.get_bind_group()
    }
}

impl Geometry for Curve {
    fn get_bind_group_object_mut(&mut self) -> &mut GeometryBindGroupObject {
        &mut self.bind_group_object
    }
}
