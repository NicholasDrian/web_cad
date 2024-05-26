// currently using a sub optimal algorithm.
// The depth is optimal but the work n log n.
// The optimal algorithm seems too complex to be worth the meager savings
use std::rc::Rc;

use wgpu::util::DeviceExt;

use crate::render::renderer::Renderer;

pub struct PrefixSumGenerator {
    renderer: Rc<Renderer>,
    prefix_sum_bind_group_layout: wgpu::BindGroupLayout,
    segmentation_bind_group_layout: wgpu::BindGroupLayout,
    prefix_sum_pipeline: wgpu::ComputePipeline,
    segmentation_pipeline: wgpu::ComputePipeline,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct PrefixSumUniform {
    offset: u32,
}

impl PrefixSumGenerator {
    pub fn new(renderer: Rc<Renderer>) -> Self {
        let device = renderer.get_device();
        let prefix_sum_shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("prefix sum"),
            source: wgpu::ShaderSource::Wgsl(include_str!("prefix_sum.wgsl").into()),
        });
        let segmentation_shader_module =
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("prefix sum"),
                source: wgpu::ShaderSource::Wgsl(
                    include_str!("prefix_sum_segmentation.wgsl").into(),
                ),
            });

        let prefix_sum_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("prefix sum"),
                entries: &[
                    // Params
                    crate::utils::compute_uniform_bind_group_layout_entry(0),
                    // Values
                    crate::utils::compute_buffer_bind_group_layout_entry(1, true),
                    // Next
                    crate::utils::compute_buffer_bind_group_layout_entry(2, false),
                ],
            });

        let segmentation_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("prefix sum segmentation"),
                entries: &[
                    // Prefix sum
                    crate::utils::compute_buffer_bind_group_layout_entry(0, true),
                    // Segment map
                    crate::utils::compute_buffer_bind_group_layout_entry(1, true),
                    // Segments
                    crate::utils::compute_buffer_bind_group_layout_entry(2, true),
                    // output
                    crate::utils::compute_buffer_bind_group_layout_entry(3, false),
                ],
            });

        let prefix_sum_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("prefix sum"),
                bind_group_layouts: &[&prefix_sum_bind_group_layout],
                push_constant_ranges: &[],
            });
        let segmentation_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("prefix sum segmentation"),
                bind_group_layouts: &[&segmentation_bind_group_layout],
                push_constant_ranges: &[],
            });

        let prefix_sum_pipeline =
            device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some("prefix sum"),
                layout: Some(&prefix_sum_pipeline_layout),
                module: &prefix_sum_shader_module,
                entry_point: "prefix_sum",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            });

        let segmentation_pipeline =
            device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some("prefix sum segmentation"),
                layout: Some(&segmentation_pipeline_layout),
                module: &segmentation_shader_module,
                entry_point: "prefix_sum_segmentation",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            });

        Self {
            renderer,
            prefix_sum_bind_group_layout,
            segmentation_bind_group_layout,
            prefix_sum_pipeline,
            segmentation_pipeline,
        }
    }

    pub fn get_prefix_sum(
        &self,
        segment_map: &wgpu::Buffer,
        segments: &wgpu::Buffer,
        values: wgpu::Buffer,
        value_count: u32,
    ) -> wgpu::Buffer {
        let device = self.renderer.get_device();

        let other_values = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("prefix sum next buffer"),
            size: value_count as u64 * std::mem::size_of::<f32>() as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("prefix sum"),
        });

        let iterations = f32::log2(value_count as f32).ceil() as u32;

        encoder.copy_buffer_to_buffer(&values, 0, &other_values, 0, 4);

        for i in 0..iterations {
            let offset = u32::pow(2, i);

            let params = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("prefix sum params"),
                contents: bytemuck::cast_slice(&[PrefixSumUniform { offset }]),
                usage: wgpu::BufferUsages::UNIFORM,
            });

            let bind_group = if i & 1 == 0 {
                device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("prefix sum even"),
                    layout: &self.prefix_sum_bind_group_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: params.as_entire_binding(),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: values.as_entire_binding(),
                        },
                        wgpu::BindGroupEntry {
                            binding: 2,
                            resource: other_values.as_entire_binding(),
                        },
                    ],
                })
            } else {
                device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("prefix sum even"),
                    layout: &self.prefix_sum_bind_group_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: params.as_entire_binding(),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: other_values.as_entire_binding(),
                        },
                        wgpu::BindGroupEntry {
                            binding: 2,
                            resource: values.as_entire_binding(),
                        },
                    ],
                })
            };

            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("prefix sum"),
                timestamp_writes: None,
            });
            compute_pass.set_pipeline(&self.prefix_sum_pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);

            compute_pass.dispatch_workgroups(value_count - offset, 1, 1);
        }

        let idx = self.renderer.get_queue().submit([encoder.finish()]);
        device.poll(wgpu::Maintain::WaitForSubmissionIndex(idx));

        if iterations & 1 == 0 {
            self.segment(&values, segment_map, segments, value_count)
        } else {
            self.segment(&other_values, segment_map, segments, value_count)
        }
    }

    fn segment(
        &self,
        prefix_sum: &wgpu::Buffer,
        segment_map: &wgpu::Buffer,
        segments: &wgpu::Buffer,
        count: u32,
    ) -> wgpu::Buffer {
        let device = self.renderer.get_device();

        let segmented_prefix_sum = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("prefix sum segmented"),
            size: count as u64 * std::mem::size_of::<f32>() as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("prefix sum segmented"),
        });

        encoder.copy_buffer_to_buffer(&prefix_sum, 0, &segmented_prefix_sum, 0, 4);

        // TODO: add 0 before prefix sum
        // WARN:
        // BUG:

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("prefix sum segmentation"),
            layout: &self.segmentation_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: prefix_sum.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: segment_map.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: segments.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: segmented_prefix_sum.as_entire_binding(),
                },
            ],
        });

        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("prefix sum segmentation"),
                timestamp_writes: None,
            });
            compute_pass.set_pipeline(&self.segmentation_pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);
            compute_pass.dispatch_workgroups(count, 1, 1);
        }

        let idx = self.renderer.get_queue().submit([encoder.finish()]);
        device.poll(wgpu::Maintain::WaitForSubmissionIndex(idx));

        segmented_prefix_sum
    }
}
