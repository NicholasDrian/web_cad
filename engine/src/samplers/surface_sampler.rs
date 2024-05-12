use std::rc::Rc;

use crate::render::renderer::Renderer;

pub struct SurfaceSampler {
    renderer: Rc<Renderer>,
}

impl SurfaceSampler {
    pub fn new(renderer: Rc<Renderer>) -> SurfaceSampler {
        SurfaceSampler { renderer }
    }
}
