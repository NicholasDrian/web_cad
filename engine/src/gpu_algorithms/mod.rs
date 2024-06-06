use std::{collections::HashMap, rc::Rc};

use crate::render::renderer::Renderer;

use self::{
    iota::create_iota_resources, prefix_sum::create_prefix_sum_resources,
    radix_sort::create_radix_sort_resources,
};

pub mod iota;
pub mod prefix_sum;
pub mod radix_sort;

#[derive(Eq, PartialEq, Hash)]
pub enum Algorithm {
    Iota,
    PrefixSum,
    RadixSort,
}

// One stop shop for generating bind group layouts and pipelines for all the algorithms
pub struct AlgorithmResources {
    renderer: Rc<Renderer>,
    resource_map: HashMap<Algorithm, (wgpu::BindGroupLayout, wgpu::ComputePipeline)>,
}

impl AlgorithmResources {
    pub fn new(renderer: Rc<Renderer>) -> Self {
        let mut resource_map = HashMap::new();
        resource_map.insert(Algorithm::PrefixSum, create_prefix_sum_resources(&renderer));
        resource_map.insert(Algorithm::Iota, create_iota_resources(&renderer));
        resource_map.insert(Algorithm::RadixSort, create_radix_sort_resources(&renderer));
        Self {
            renderer,
            resource_map,
        }
    }
    pub fn get_renderer(&self) -> &Renderer {
        &self.renderer
    }
    pub fn get_resources(
        &self,
        algo: Algorithm,
    ) -> &(wgpu::BindGroupLayout, wgpu::ComputePipeline) {
        self.resource_map.get(&algo).unwrap()
    }
}
