@group(0) @binding(1) var<uniform> paras: Params;
@group(0) @binding(0) var<buffer, read> values: array<u32>;
@group(0) @binding(2) var<buffer, read_write> next: array<u32>;

struct Params {
  iteration: u32,
}

@compute @workgroup_size(1,1,1)
fn prefix_sum(
  @builtin( global_invocation_id) id: vec3<u32>,
) {

}
