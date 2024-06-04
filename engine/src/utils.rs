macro_rules! get_instance_mut {
    ($handle:expr) => {{
        crate::instance::INSTANCES
            .lock()
            .unwrap()
            .get_mut($handle)
            .unwrap()
    }};
}

pub(crate) use get_instance_mut;

pub(crate) const fn compute_buffer_bind_group_layout_entry(
    binding: u32,
    read_only: bool,
) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::COMPUTE,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Storage { read_only },
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}
pub(crate) const fn compute_uniform_bind_group_layout_entry(
    binding: u32,
) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::COMPUTE,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}

pub(crate) fn create_compute_pipeline(
    device: &wgpu::Device,
    label: &str,
    shader_src: &str,
    bind_group_layout: &wgpu::BindGroupLayout,
    entry_point: &str,
) -> wgpu::ComputePipeline {
    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("bb buffer gen"),
        source: wgpu::ShaderSource::Wgsl(shader_src.into()),
    });
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some(label),
        bind_group_layouts: &[bind_group_layout],
        push_constant_ranges: &[],
    });

    device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some(label),
        layout: Some(&layout),
        module: &module,
        entry_point,
        compilation_options: wgpu::PipelineCompilationOptions::default(),
    })
}

// dumps a buffer with COPY_SRC usage
pub(crate) async fn dump_buffer_of_u32(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    buffer: &wgpu::Buffer,
    offset: u32,
    count: u32,
) {
    let intermediate = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("debug print intermediate"),
        size: count as u64 * 4,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("debug print"),
    });
    encoder.copy_buffer_to_buffer(
        buffer,
        offset as u64 * std::mem::size_of::<u32>() as u64,
        &intermediate,
        0,
        count as u64 * std::mem::size_of::<u32>() as u64,
    );

    let idx = queue.submit([encoder.finish()]);
    device.poll(wgpu::Maintain::WaitForSubmissionIndex(idx));

    let (sender, receiver) = futures::channel::oneshot::channel();

    let slice = intermediate.slice(..);
    slice.map_async(wgpu::MapMode::Read, |result| {
        let _ = sender.send(result);
    });

    receiver
        .await
        .expect("communication failed")
        .expect("buffer reading failed");

    let bytes: &[u8] = &slice.get_mapped_range();

    for i in 0..count {
        if i % 12 == 0 {
            log::info!("\n");
        }
        let num = u32::from_le_bytes(
            bytes[(i as usize * 4)..(i as usize * 4 + 4)]
                .try_into()
                .unwrap(),
        );
        log::info!("{:?} - buffer num = {:?}", i, num);
    }
}
