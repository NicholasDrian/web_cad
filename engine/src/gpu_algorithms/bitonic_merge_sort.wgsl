@group(0) @binding(0) var<uniform> params: Params;
// TODO: parameterized key type and value type
@group(0) @binding(1) var<storage, read_write> keys: array<MyU64>;
@group(0) @binding(2) var<storage, read_write> values: array<u32>;

struct MyU64 {
  upper_bits: u32,
  lower_bits: u32,
}

struct Params {
  sort_size: u32,
  step_size: u32,
  num_values: u32,
}

fn swap_indices(i: u32, j: u32) {
  let temp1 = keys[i];
  keys[i] = keys[j];
  keys[j] = temp1;

  let temp2 = values[i];
  values[i] = values[j];
  values[j] = temp2;
}

// i > j
fn is_greater(i: u32, j: u32) -> bool {
  if (keys[i].upper_bits > keys[j].upper_bits) {
    return true;
  }
  if (keys[i].upper_bits < keys[j].upper_bits) {
    return false;
  }
  return keys[i].lower_bits > keys[j].lower_bits;
}

fn make_ascend(i: u32, j: u32, invert: bool) {

  // if i or j are out of bounds, ignore them
  // TODO: try to elminate this for speed!
  // if num workgroups is correctly setup, shouldnt need this.
  if (i >= params.num_values || j >= params.num_values){ return; }

  var should_swap = is_greater(i, j);
  if (invert) { should_swap = !should_swap;}
  if (should_swap) {
    swap_indices(i, j);
  }
}

@compute @workgroup_size(1,1,1) 
fn main(
  @builtin(global_invocation_id) id: vec3<u32>,
) {
  let sort_num = id.x / (params.sort_size / 2);
  let sort_start = sort_num * params.sort_size;
  let position_in_sort = id.x % (params.sort_size / 2);
  make_ascend(
    sort_start + position_in_sort,
    sort_start + position_in_sort + params.step_size,
    (sort_num & 1) == 1,
  ); 
}
