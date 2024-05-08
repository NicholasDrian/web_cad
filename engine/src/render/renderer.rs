pub struct Renderer {}

impl Renderer {
    pub async fn new() -> Renderer {
        let gpu = web_sys::window().unwrap().navigator().gpu();

        Renderer {}
    }
}
