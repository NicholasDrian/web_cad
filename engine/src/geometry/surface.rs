use crate::{
    gpu_acceleration_structures::mesh_bbh::{mesh_bbh_generator::MeshBBHGenerator, MeshBBH},
    gpu_samplers::{params::SAMPLES_PER_SEGMENT, surface_sampler::SurfaceSampler},
    math::linear_algebra::{vec3::Vec3, vec4::Vec4},
};
use std::rc::Rc;

use super::{bind_group::GeometryBindGroupObject, utils::default_knot_vector, Geometry};

pub struct Surface {
    surface_sampler: Rc<SurfaceSampler>,
    bbh_generator: Rc<MeshBBHGenerator>,
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
    bbh: Option<MeshBBH>,
}

impl Surface {
    pub async fn new(
        surface_sampler: Rc<SurfaceSampler>,
        bbh_generator: Rc<MeshBBHGenerator>,
        control_count_u: u32,
        control_count_v: u32,
        degree_u: u32,
        degree_v: u32,
        controls: Vec<Vec3>,
        weights: &[f32],
        knots_u: &[f32],
        knots_v: &[f32],
        with_bbh: bool,
    ) -> Self {
        let knots_u = if knots_u.is_empty() {
            default_knot_vector(control_count_u as usize, degree_u)
        } else {
            knots_u.to_vec()
        };
        let knots_v = if knots_v.is_empty() {
            default_knot_vector(control_count_v as usize, degree_v)
        } else {
            knots_v.to_vec()
        };
        let weights = if weights.is_empty() {
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
        let bbh = if with_bbh {
            Some(
                bbh_generator
                    .generate_mesh_bbh_fast_build(
                        &vertex_buffer,
                        sample_count_u * sample_count_v,
                        &index_buffer,
                        index_count,
                    )
                    .await,
            )
        } else {
            None
        };

        Self {
            surface_sampler,
            bbh_generator,
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
            bbh,
        }
    }

    /// Control count u and v must not change
    /// TODO: update to be able to change these
    pub async fn update_params(
        &mut self,
        degree_u: u32,
        degree_v: u32,
        controls: Vec<Vec3>,
        weights: &[f32],
        knots_u: &[f32],
        knots_v: &[f32],
        with_bbh: bool,
    ) {
        self.degree_u = degree_u;
        self.degree_v = degree_v;
        if weights.len() != 0 {
            self.weights = weights.to_vec();
        }
        if controls.len() != 0 {
            self.controls = controls;
        }
        if knots_u.len() != 0 {
            self.knots_u = knots_u.to_vec();
        }
        if knots_v.len() != 0 {
            self.knots_v = knots_v.to_vec();
        }

        let weighted_controls: Vec<Vec4> = self
            .controls
            .iter()
            .zip(self.weights.iter())
            .map(|(control, weight)| Vec4 {
                x: control.x * weight,
                y: control.y * weight,
                z: control.z * weight,
                w: *weight,
            })
            .collect();
        let (_, vertex_buffer) = self.surface_sampler.sample_surface(
            self.degree_u,
            self.degree_v,
            &weighted_controls[..],
            self.control_count_u,
            self.control_count_v,
            &self.knots_u[..],
            &self.knots_v[..],
        );
        self.vertex_buffer = vertex_buffer;

        let sample_count_u = SAMPLES_PER_SEGMENT * (self.control_count_u - 1) + 1;
        let sample_count_v = SAMPLES_PER_SEGMENT * (self.control_count_v - 1) + 1;

        self.bbh = if with_bbh {
            Some(
                self.bbh_generator
                    .generate_mesh_bbh_fast_trace(
                        &self.vertex_buffer,
                        sample_count_u * sample_count_v,
                        &self.index_buffer,
                        self.index_count,
                    )
                    .await,
            )
        } else {
            None
        };
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

    /// Unsafe
    pub fn get_bbh(&self) -> Option<&MeshBBH> {
        if self.bbh.is_some() {
            Some(self.bbh.as_ref().unwrap())
        } else {
            None
        }
    }
}

impl Geometry for Surface {
    fn get_bind_group_object_mut(&mut self) -> &mut GeometryBindGroupObject {
        &mut self.bind_group_object
    }
}
