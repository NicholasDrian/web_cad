pub fn create_and_write_buffer(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    label: &'static str,
    usage: wgpu::BufferUsages,
    mapped_at_creation: bool,
    data: &[u8],
) -> wgpu::Buffer {
    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size: data.len() as u64,
        usage,
        mapped_at_creation,
    });
    queue.write_buffer(&buffer, 0, data);
    buffer
}
