@group(0) @binding(0) var<uniform> params: Params,
// TODO: parameterized key type and value type
@group(0) @binding(1) var<storage, read_write> keys: array<i64>,
@group(0) @binding(2) var<storage, read_write> values: array<u32>,

struct Params {
  sort_size: u32,
  step_size: u32,
}

fn swap(i: u32, j: u32) {
  temp1 = keys[i];
  keys[i] = keys[j];
  keys[j] = temp1;

  temp2 = values[i];
  values[i] = values[j];
  values[j] = temp2;
}

// swap if the first key is larger
fn make_ascend(i: u32, j: u32) {
  if (keys[i] - keys[j] > 0) {
    swap(i, j);
  }
}

@compute @workgroup_size(1,1,1) 
fn main(
  @builtin(global_invocation_id) id: vec3<u32>,
  @builtin(num_workgroups) size: vec3<u32>,
) {
  TODO:
}
