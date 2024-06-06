@group(0) @binding(0) var<uniform> params: Params,
// TODO: parameterized key type and value type
@group(0) @binding(1) var<storage, read_write> keys: array<i64>,
@group(0) @binding(2) var<storage, read_write> values: array<u32>,

struct Params {
  sort_size: u32,
  step_size: u32,
  num_values: u32,
}

fn swap_indices(i: u32, j: u32) {
  temp1 = keys[i];
  keys[i] = keys[j];
  keys[j] = temp1;

  temp2 = values[i];
  values[i] = values[j];
  values[j] = temp2;
}

// sign must be 1 or -1
// if sign == 1: swap if the first key is larger
// if sign == -1: swap if the first key is smaller
fn make_ascend(i: u32, j: u32, sign: i64) {

  // if i or j are out of bounds, ignore them
  // TODO: try to elminate this for speed!
  // if num workgroups is correctly setup, shouldnt need this.
  if (i >= num_values || j >= num_values) return;

  if ((keys[i] - keys[j]) * sign > 0) {
    swap_indices(i, j);
  }
}

@compute @workgroup_size(1,1,1) 
fn main(
  @builtin(global_invocation_id) id: vec3<u32>,
) {
  let sort_num = id.x / (params.sort_size / 2);
  var sort_start = sort_num * sort_size;
  var sort_end = sort_start + sort_size - 1; // inclusive

  if (sort_num & 1 == 1) {
    let temp = sort_end; 
    sort_end = sort_start;
    sort_start = temp;
  }

  // think about this TODO: 
  let position_in_sort = id.x % sort_num;
}
