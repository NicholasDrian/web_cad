// buid bbh and cull leaf segments
// will do this linearly for now
// need to return updated segment count to gpu

@group(0) @binding(0) var<uniform> params: Parmas;

struct Params {
  tree_size: u32,
}

// 

@compute @workgroup_size(1,1,1)
fn main() {

}

