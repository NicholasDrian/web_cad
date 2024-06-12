@group(0) @binding(0) var<uniform> params: Params;
// TODO: parameterized key type and value type
@group(0) @binding(1) var<storage, read_write> keys: array<MyI64>;
@group(0) @binding(2) var<storage, read_write> values: array<u32>;

struct MyI64 {
  i32,
  u32,
}

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
fn make_ascend(i: u32, j: u32, sign: i32) {

  // if i or j are out of bounds, ignore them
  // TODO: try to elminate this for speed!
  // if num workgroups is correctly setup, shouldnt need this.
  if (i >= num_values || j >= num_values){ return; }

  if ((keys[i] - keys[j]) * sign > 0) {
    swap_indices(i, j);
  }
}

@compute @workgroup_size(1,1,1) 
fn main(
  @builtin(global_invocation_id) id: vec3<u32>,
) {

  // number of soon to be sorted segment
  let sort_num = id.x / (params.sort_size / 2);
  let sort_start = sort_num * sort_size;
  let position_in_sort = id.x % (params.sort_size / 2);
  var sign: i32 = 1;
  if ((sort_num & 1) == 1) { sign = -1; }
  make_ascend(
    sort_start + position_in_sort,
    sort_start + position_in_sort + params.sort_size / 2,
    sign
  ); 
}
