use std::rc::Rc;

use crate::{math::linear_algebra::vec3::Vec3, render::renderer::Renderer};

pub struct CurveSampler {
    renderer: Rc<Renderer>,
    shader: wgpu::ShaderModule,
    bind_group_layout: wgpu::BindGroupLayout,
    uniform_buffer: wgpu::Buffer,
    pipeline: wgpu::ComputePipeline,
}

impl CurveSampler {
    pub fn new(renderer: Rc<Renderer>) -> CurveSampler {
        let shader = renderer
            .get_device()
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("curve sampler compute shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("curve_sampler.wgsl").into()),
            });

        let bind_group_layout =
            renderer
                .get_device()
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("curve sampler bind group layout"),

                    entries: &[
                        // Params
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                        // Controls
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage { read_only: true },
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                        // Knots
                        wgpu::BindGroupLayoutEntry {
                            binding: 2,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage { read_only: true },
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                        // Basis Funcs
                        wgpu::BindGroupLayoutEntry {
                            binding: 3,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage { read_only: false },
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                        // Samples
                        wgpu::BindGroupLayoutEntry {
                            binding: 4,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage { read_only: false },
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                    ],
                });

        let uniform_buffer = renderer
            .get_device()
            .create_buffer(&wgpu::BufferDescriptor {
                label: Some("curve sampler uniform buffer"),
                size: 12, // TODO: use sizeof
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });

        let pipeline_layout =
            renderer
                .get_device()
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("curve sampler pipeline layout"),
                    bind_group_layouts: &[&bind_group_layout],
                    push_constant_ranges: &[],
                });

        let pipeline =
            renderer
                .get_device()
                .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    label: Some("curve sampler pipeline"),
                    layout: Some(&pipeline_layout),
                    module: &shader,
                    entry_point: "main",
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                });

        CurveSampler {
            renderer,
            shader,
            bind_group_layout,
            uniform_buffer,
            pipeline,
        }
    }

    pub fn sample_curve(
        &self,
        degree: u32,
        controls: &[Vec3],
        weights: &[f32],
        knots: &[f32],
    ) -> wgpu::Buffer {
        todo!()
    }
}
