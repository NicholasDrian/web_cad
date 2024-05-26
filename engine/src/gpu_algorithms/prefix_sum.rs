// TODO: improve using https://developer.nvidia.com/gpugems/gpugems3/part-vi-gpu-computing/chapter-39-parallel-prefix-sum-scan-cuda
pub fn get_prefix_sum_naieve(
    segments: &wgpu::Buffer,
    segment_count: u32,
    values: &wgpu::Buffer,
    value_count: u32,
) -> wgpu::Buffer {
    todo!()
}
