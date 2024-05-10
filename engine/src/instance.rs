use std::sync::{Arc, Mutex};
use std::{collections::HashMap, rc::Rc};

use crate::{render::renderer::Renderer, scene::scene::Scene, viewport::viewport::Viewport};
use web_sys::HtmlCanvasElement;

pub type InstanceHandle = u64;
static mut INSTANCE_HANDLE_GENERATOR: Mutex<InstanceHandle> = Mutex::new(0u64);
pub fn new_instance_handle() -> InstanceHandle {
    unsafe {
        let mut changer = INSTANCE_HANDLE_GENERATOR.lock().unwrap();
        *changer += 1u64;
        *changer
    }
}

lazy_static! {
    pub static ref INSTANCES: Mutex<HashMap<InstanceHandle, Instance>> = Mutex::new(HashMap::new());
}

pub struct Instance {
    // TODO: replace rc with lifetime
    renderer: Rc<Renderer>,
    scene: Scene,
    viewports: Vec<Viewport>,
}
unsafe impl Send for Instance {}

impl Instance {
    pub async fn create(canvases: &[HtmlCanvasElement]) -> InstanceHandle {
        let renderer = Rc::new(Renderer::new().await);
        let scene = Scene::new();
        let viewports = canvases
            .iter()
            .map(|canvas| Viewport::new(canvas.clone(), renderer.clone()))
            .collect();

        let instance = Instance {
            renderer,
            scene,
            viewports,
        };
        instance.draw();

        let instance_handle = new_instance_handle();

        let mut instances_changer = INSTANCES.lock().unwrap();
        (*instances_changer).insert(instance_handle, instance);

        instance_handle
    }

    pub fn draw(&self) {
        self.renderer.render(&self.scene, &self.viewports);
    }
}
