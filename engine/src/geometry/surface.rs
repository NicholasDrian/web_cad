use crate::{
    math::linear_algebra::{vec3::Vec3, vec4::Vec4},
    samplers::{params::SAMPLES_PER_SEGMENT, surface_sampler::SurfaceSampler},
};

use super::{bind_group::GeometryBindGroupObject, geometry::Geometry, utils::default_knot_vector};

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
    index_count: u32,
    index_buffer: wgpu::Buffer,
    bind_group_object: GeometryBindGroupObject,
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
        let knots_v = if knots_v.len() == 0 {
            default_knot_vector(control_count_v as usize, degree_v)
        } else {
            knots_v.to_vec()
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
        let (index_buffer, vertex_buffer) = surface_sampler.sample_surface(
            degree_u,
            degree_v,
            &weighted_controls[..],
            control_count_u,
            control_count_v,
            &knots_u[..],
            &knots_v[..],
        );
        let sample_count_u = SAMPLES_PER_SEGMENT * (control_count_u - 1) + 1;
        let sample_count_v = SAMPLES_PER_SEGMENT * (control_count_v - 1) + 1;
        let index_count = (sample_count_u - 1) * (sample_count_v - 1) * 6;
        let bind_group_object = GeometryBindGroupObject::new(surface_sampler.get_renderer());

        Self {
            controls,
            control_count_u,
            control_count_v,
            degree_u,
            degree_v,
            weights,
            knots_u,
            knots_v,
            vertex_buffer,
            index_count,
            index_buffer,
            bind_group_object,
        }
    }
    pub fn get_index_buffer(&self) -> &wgpu::Buffer {
        &self.index_buffer
    }
    pub fn get_vertex_buffer(&self) -> &wgpu::Buffer {
        &self.vertex_buffer
    }
    pub fn get_index_count(&self) -> u32 {
        self.index_count
    }
    pub fn get_bind_group(&self) -> &wgpu::BindGroup {
        self.bind_group_object.get_bind_group()
    }
}

impl Geometry for Surface {
    fn get_bind_group_object_mut(&mut self) -> &mut GeometryBindGroupObject {
        &mut self.bind_group_object
    }
}
