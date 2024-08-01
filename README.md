RN:

instance is not thread safe. 
  - place instance behind mutex (arc<mutex<Instance>>)
  - instance is only used to create renderers and viewports

devices and queues are thread safe
  - those can stay in instance

solution
  - make one global engine instance, pretty much a wrapper around wgpu::Instance
    - global instance should be protected with arc<mutex<Instance>>
  - make app class
    - should contain queue, and device, 
    - should be automatically send + sync
  - make renderer and scene belong to app
    - these should have a reference to their parent app to access device and queue (lifetimes are tricky :/ )
  - tada  

Plan of execution: 
  - break everything
  - fix everything




This repo contains a number of things:

- A CAD library/engine for web apps
  - Written in Rust, compiled to WASM, using WebGPU, targeting modern browsers.
  - Designed to create and keep all data on the GPU.
    -Eliminates data streaming.
  - 50,000,000 samples per second of degree {u: 10, v: 10} surface with 100 x 100 control points
  - currently a WIP
    - benchmarks, examples, and docs coming soon!
- Basic examples of how to use the engine.
  - to run an example, use ./run [example to run]
  - for example, to run example_01, type ./run example_01
  - example_01
    - hello_cad
  - example_02
    - hello surface
  - example_03
    - hello camera: todo
- CAD software that uses the engine.
  - Soon to come

This project is in its infancy, stay tuned for updates

BUG: writing over data that we dont own ( due to worgroups size not being perfect cube)
TODO_FIRST: add model transform to debug bbh :)
TODO: factor out GPU device from renderer
TODO: nice web page
TODO: event loop js
TODO: make samplers and renderer async better
TODO: start removing unwraps
TODO: gumball as aditional feature
TODO: raytracing as aditional feature
TODO: frustum tracing as aditional feature
TODO: 42 warnings, lest chop this in half
TODO: remove copy src flags used for debug
TODO: add workgroup size

