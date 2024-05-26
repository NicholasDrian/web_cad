@group(0) @binding(0) var<storage, read> prefix_sum: array<u32>;
@group(0) @binding(1) var<storage, read> segment_map: array<u32>;
@group(0) @binding(2) var<storage, read> segments: array<u32>;
@group(0) @binding(3) var<storage, read_write> segmented_prefix_sum: array<u32>;


@compute @workgroup_size(1,1,1)
fn segment_prefix_sum(
  @builtin(global_invocation_id) id: vec3<u32>,
) {
  // WRONG
  // ERROR:
  // BUG:
  // idx should equal id.x

  // index into values
  let idx = id.x + segments[1];

  // current segment of index
  let seg = segment_map[idx];

  // total of previous segment
  let prev_total = prefix_sum[segments[seg] - 1];

  segmented_prefix_sum[idx] -= prev_total;

}
