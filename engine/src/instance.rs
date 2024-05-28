use std::sync::Mutex;
use std::{collections::HashMap, rc::Rc};

use crate::gpu_acceleration_structures::mesh_bbh_generator::MeshBBHGenerator;
use crate::gpu_algorithms::AlgorithmResources;
use crate::gpu_samplers::curve_sampler::CurveSampler;
use crate::gpu_samplers::surface_sampler::SurfaceSampler;
use crate::scene::scene_interface::Scene;
use crate::viewport::viewport_interface::Viewport;
use crate::{render::renderer::Renderer, scene::SceneInternal, viewport::ViewportInternal};
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
    algorithm_resources: Rc<AlgorithmResources>,
    scenes: HashMap<Handle, SceneInternal>,
    viewports: HashMap<Handle, ViewportInternal>,
    curve_sampler: Rc<CurveSampler>,
    surface_sampler: Rc<SurfaceSampler>,
    mesh_bbh_generator: MeshBBHGenerator,
}
unsafe impl Send for InstanceInternal {}

impl InstanceInternal {
    pub async fn create() -> Handle {
        let renderer = Rc::new(Renderer::new().await);
        let algorithm_resources = Rc::new(AlgorithmResources::new(renderer.clone()));
        let curve_sampler = Rc::new(CurveSampler::new(renderer.clone()));
        let surface_sampler = Rc::new(SurfaceSampler::new(renderer.clone()));
        let mesh_bbh_generator =
            MeshBBHGenerator::new(renderer.clone(), algorithm_resources.clone());
        let instance = InstanceInternal {
            scenes: HashMap::new(),
            viewports: HashMap::new(),
            curve_sampler,
            surface_sampler,
            renderer,
            algorithm_resources,
            mesh_bbh_generator,
        };

        let handle = new_handle();
        (*INSTANCES.lock().unwrap()).insert(handle, instance);
        handle
    }

    pub fn create_viewport(&mut self, canvas: HtmlCanvasElement) -> Handle {
        let viewport = ViewportInternal::new(canvas, self.renderer.clone(), 4);
        let handle = new_handle();
        self.viewports.insert(handle, viewport);
        handle
    }
    pub fn create_scene(&mut self) -> Handle {
        let scene = SceneInternal::new();
        let handle = new_handle();
        self.scenes.insert(handle, scene);
        handle
    }

    pub fn draw_scene_to_viewport(&self, scene: &Scene, viewport: &Viewport) {
        let viewport = self.viewports.get(&viewport.get_handle()).unwrap();
        let scene = self.scenes.get(&scene.get_handle()).unwrap();
        self.renderer.render(scene, viewport);
    }

    pub fn draw_scene_to_all_viewports(&self, scene: &Scene) {
        let scene = self.scenes.get(&scene.get_handle()).unwrap();
        for (_, viewport) in self.viewports.iter() {
            self.renderer.render(scene, viewport);
        }
    }

    pub fn get_viewport_mut(&mut self, viewport_handle: Handle) -> &mut ViewportInternal {
        self.viewports.get_mut(&viewport_handle).unwrap()
    }
    pub fn get_scene_mut(&mut self, scene_handle: Handle) -> &mut SceneInternal {
        self.scenes.get_mut(&scene_handle).unwrap()
    }
    pub fn get_renderer(&self) -> Rc<Renderer> {
        self.renderer.clone()
    }
    pub fn get_curve_sampler(&self) -> &CurveSampler {
        &self.curve_sampler
    }
    pub fn get_surface_sampler(&self) -> Rc<SurfaceSampler> {
        self.surface_sampler.clone()
    }
    pub fn get_mesh_bbh_generator(&self) -> &MeshBBHGenerator {
        &self.mesh_bbh_generator
    }
}
