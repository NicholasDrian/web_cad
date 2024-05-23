use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

use crate::{
    instance::{Handle, InstanceInternal},
    scene::scene_interface::Scene,
    utils::get_instance_mut,
    viewport::viewport_interface::Viewport,
};

/// Instance wrapper that is available in JS
#[wasm_bindgen]
pub struct WebCadInstance {
    /// Handle to interanal wasm instance
    handle: Handle,
}

#[wasm_bindgen]
impl WebCadInstance {
    // Not using contructor because async constructor doesnt play well with wasm_bindgen.
    #[wasm_bindgen]
    pub async fn new_instance() -> WebCadInstance {
        WebCadInstance {
            handle: InstanceInternal::create().await,
        }
    }

    #[wasm_bindgen]
    pub fn create_viewport(&self, canvas: HtmlCanvasElement) -> Viewport {
        Viewport::new(
            self.handle,
            get_instance_mut!(&self.handle).create_viewport(canvas),
        )
    }

    #[wasm_bindgen]
    pub fn create_scene(&self) -> Scene {
        Scene::new(self.handle, get_instance_mut!(&self.handle).create_scene())
    }

    #[wasm_bindgen]
    pub fn draw_scene_to_all_viewports(&self, scene: &Scene) {
        get_instance_mut!(&self.handle).draw_scene_to_all_viewports(scene);
    }

    #[wasm_bindgen]
    pub fn draw_scene_to_viewport(&self, scene: &Scene, viewport: &Viewport) {
        get_instance_mut!(&self.handle).draw_scene_to_viewport(scene, viewport);
    }
}
