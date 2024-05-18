use std::collections::HashMap;

use wgpu::util::{BufferInitDescriptor, DeviceExt};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct IndexGeneratorUnifroms {
    count_u: u32,
}

pub struct IndexBufferGenerator {
    pipeline: wgpu::ComputePipeline,
    bind_group_layout: wgpu::BindGroupLayout,
    memo: HashMap<u64, wgpu::Buffer>,
}
impl IndexBufferGenerator {
    pub fn new(device: &wgpu::Device) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("index buffer generator compute shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("index_buffer_generator.wgsl").into()),
        });
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("index buffer generator bind group layout"),

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
                // Output
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
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

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("index buffer generator compute pipeline layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("index buffer generator compute pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: "main",
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        });
        Self {
            pipeline,
            bind_group_layout,
            memo: HashMap::new(),
        }
    }
    // TODO: use self.memo
    pub fn get_index_buffer(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        count_u: u32,
        count_v: u32,
    ) -> wgpu::Buffer {
        // TODO:
        /*
        let key = count_u as u64 | (count_v as u64).rotate_left(32);
        if let index_buffer = self.memo.get(&key) {
            log::warn!("YAY, succseefull memo, please delete this new");
            return *index_buffer.unwrap();
        }
        */

        let index_buffer: wgpu::Buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("surface index buffer"),
            size: ((count_u - 1) * (count_v - 1) * 6 * std::mem::size_of::<u32>() as u32) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::INDEX,
            mapped_at_creation: false,
        });

        let uniform_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("index buffer generatore uniform buffer"),
            contents: bytemuck::cast_slice(&[IndexGeneratorUnifroms { count_u }]),
            usage: wgpu::BufferUsages::UNIFORM,
        });

        let bind_group: wgpu::BindGroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("index buffer generator bind group"),
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: index_buffer.as_entire_binding(),
                },
            ],
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("index buffer generator encoder"),
        });

        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("index buffer generator compute pass"),
                timestamp_writes: None,
            });

            compute_pass.set_pipeline(&self.pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);
            compute_pass.dispatch_workgroups(1, count_v - 1, 1);
        }

        let idx = queue.submit([encoder.finish()]);

        device.poll(wgpu::Maintain::WaitForSubmissionIndex(idx));

        // TODO:
        //self.memo.insert(key, index_buffer);
        index_buffer
    }
}
