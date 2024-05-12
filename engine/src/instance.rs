use std::sync::Mutex;
use std::{collections::HashMap, rc::Rc};

use crate::scene::scene_interface::Scene;
use crate::{
    render::renderer::Renderer, scene::scene::SceneInternal, viewport::viewport::Viewport,
};
use web_sys::HtmlCanvasElement;

pub type Handle = u64;
static mut HANDLE_GENERATOR: Mutex<Handle> = Mutex::new(0u64);
pub fn new_handle() -> Handle {
    unsafe {
        let mut changer = HANDLE_GENERATOR.lock().unwrap();
        *changer += 1u64;
        *changer
    }
}

lazy_static! {
    pub static ref INSTANCES: Mutex<HashMap<Handle, InstanceInternal>> = Mutex::new(HashMap::new());
}

pub struct InstanceInternal {
    renderer: Rc<Renderer>,
    scenes: HashMap<Handle, SceneInternal>,
    viewports: HashMap<Handle, Viewport>,
}
unsafe impl Send for InstanceInternal {}

impl InstanceInternal {
    pub async fn create() -> Handle {
        let instance = InstanceInternal {
            renderer: Rc::new(Renderer::new().await),
            scenes: HashMap::new(),
            viewports: HashMap::new(),
        };

        let handle = new_handle();
        (*INSTANCES.lock().unwrap()).insert(handle, instance);
        handle
    }

    pub fn add_viewport(&mut self, canvas: HtmlCanvasElement) -> Handle {
        let viewport = Viewport::new(canvas, self.renderer.clone());
        let handle = new_handle();
        self.viewports.insert(handle, viewport);
        handle
    }
    pub fn add_scene(&mut self) -> Handle {
        let scene = SceneInternal::new();
        let handle = new_handle();
        self.scenes.insert(handle, scene);
        handle
    }

    pub fn draw_scene_to_viewport(&self, scene: Scene, viewport_handle: Handle) {
        let viewport = self.viewports.get(&viewport_handle).unwrap();
        let scene = self.scenes.get(&scene.get_handle()).unwrap();
        self.renderer.render(scene, viewport);
    }

    pub fn draw_scene_to_all_viewports(&self, scene: Scene) {
        let scene = self.scenes.get(&scene.get_handle()).unwrap();
        for (_, viewport) in self.viewports.iter() {
            self.renderer.render(scene, &viewport);
        }
    }

    pub fn get_scene_mut(&mut self, scene_handle: Handle) -> &mut SceneInternal {
        self.scenes.get_mut(&scene_handle).unwrap()
    }

    pub fn get_renderer(&self) -> Rc<Renderer> {
        self.renderer.clone()
    }
}
