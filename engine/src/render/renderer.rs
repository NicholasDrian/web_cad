use web_sys::{GpuAdapter, GpuDevice};

pub struct Renderer {
    device: GpuDevice,
}

impl Renderer {
    pub async fn new() -> Renderer {
        let gpu = web_sys::window().unwrap().navigator().gpu();

        // TODO: fail gracefully
        let adapter: GpuAdapter = wasm_bindgen_futures::JsFuture::from(gpu.request_adapter())
            .await
            .unwrap()
            .into();
        let device: GpuDevice = wasm_bindgen_futures::JsFuture::from(adapter.request_device())
            .await
            .unwrap()
            .into();

        Renderer { device }
    }
}
