use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

use crate::{
    instance::{Handle, InstanceInternal, INSTANCES},
    scene::scene_interface::Scene,
};

/// Instance that wrapper that is available in JS
#[wasm_bindgen]
pub struct Instance {
    /// Handle to interanal wasm instance
    handle: Handle,
}

#[wasm_bindgen]
impl Instance {
    #[wasm_bindgen(constructor)]
    pub async fn new() -> Instance {
        Instance {
            handle: InstanceInternal::create().await,
        }
    }

    #[wasm_bindgen]
    pub fn create_viewport(&self, canvas: HtmlCanvasElement) -> Handle {
        INSTANCES
            .lock()
            .unwrap()
            .get_mut(&self.handle)
            .unwrap()
            .create_viewport(canvas)
    }

    #[wasm_bindgen]
    pub fn create_scene(&self) -> Scene {
        Scene::new(
            self.handle,
            INSTANCES
                .lock()
                .unwrap()
                .get_mut(&self.handle)
                .unwrap()
                .create_scene(),
        )
    }

    #[wasm_bindgen]
    pub fn draw_scene_to_all_viewports(&self, scene: Scene) {
        INSTANCES
            .lock()
            .unwrap()
            .get(&self.handle)
            .unwrap()
            .draw_scene_to_all_viewports(scene);
    }

    #[wasm_bindgen]
    pub fn draw_scene_to_viewport(&self, scene: Scene, viewport_handle: Handle) {
        INSTANCES
            .lock()
            .unwrap()
            .get(&self.handle)
            .unwrap()
            .draw_scene_to_viewport(scene, viewport_handle);
    }
}
