<h1><b><u>web_cad</u></b> - A Light Weight Geometry Engine For Web Apps</h2>

<br>
<h2>What is It?</h2>
web_cad is a geometry engine built on top of WebGPU compute shaders. It is designed to be blazingly fast, light as a feather, and reliable as duct tape. Many of the core features are working, but there is still much to do.

The engine is written in Rust and WGSL, compiled to WASM and SPIR-V, and exposes a js API.

Done:
- Surface Sampler
- Curve Sampler
- Acceleration Structures
Coming:
- Ray Tracing
- Frustum Tracing
- Reparameterizing algos
- Higher level commands
- Materiality
- Stable API
- Optimization


<h2>Examples:</h2>
<h4 style="margin:0px; padding:0px;"> Click <a href="https://nicholasdrian.github.io/web_cad/examples/01_hello_cad/dist/index.html"> <u>HERE</u> </a> to run hello cad</h4>
<h4 style="margin:0px; padding:0px;"> Click <a href="https://nicholasdrian.github.io/web_cad/examples/02_hello_surface/dist/index.html"> <u>HERE</u> </a> to run hello surface</h4>
<h4 style="margin:0px; padding:0px;"> Click <a href="https://nicholasdrian.github.io/web_cad/examples/03_hello_camera/dist/index.html"> <u>HERE</u> </a> to run hello camera</h4>
<h4 style="margin:0px; padding:0px;"> Click <a href="https://nicholasdrian.github.io/web_cad/examples/04_hello_bbh/dist/index.html"> <u>HERE</u> </a> to run hello bbh</h4>

<br>

<h4 style="margin:0px; padding:0px;"> Click <a href="https://nicholasdrian.github.io/web_cad/docs/working_notes.md"> <u>HERE</u> </a> to view dev notes</h4>

