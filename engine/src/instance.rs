use std::sync::Mutex;
use std::{collections::HashMap, rc::Rc};

use crate::{render::renderer::Renderer, scene::scene::Scene, viewport::viewport::Viewport};
use web_sys::HtmlCanvasElement;

/*
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
*/

static mut INSTANCE: Mutex<Option<Instance>> = Mutex::new(None);

pub struct Instance {
    // TODO: replace rc with lifetime
    renderer: Rc<Renderer>,
    scene: Scene,
    viewports: Vec<Viewport>,
}

impl Instance {
    pub async fn create(canvases: &Vec<HtmlCanvasElement>) {
        unsafe {
            let instance_changer = INSTANCE.lock().unwrap();
            if (*instance_changer).is_some() {
                panic!("cannot create multiple instances... yet");
            }
        }

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

        unsafe {
            let mut instance_changer = INSTANCE.lock().unwrap();
            *instance_changer = Some(instance);
        }
    }

    pub fn draw(&self) {
        self.renderer.render(&self.scene, &self.viewports);
    }
}
