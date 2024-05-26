@group(0) @binding(0) var<storage, read> prefix_sum: array<u32>;
@group(0) @binding(1) var<storage, read> segments: array<u32>;
@group(0) @binding(2) var<storage, read_write> segmented_prefix_sum: array<u32>;


@compute @workgroup_size(1,1,1)
fn segment_prefix_sum(
  @builtin(global_invocation_id) id: vec3<u32>,
) {

}
