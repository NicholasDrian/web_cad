use crate::{render::renderer::Renderer, scene::scene::Scene, viewport::viewport::Viewport};

pub struct Instance {
    renderer: Renderer,
    scene: Scene,
    viewports: Vec<Viewport>,
}

impl Instance {}
