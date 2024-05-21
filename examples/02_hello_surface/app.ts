import { Instance } from '../../engine/pkg'

console.log("running example 2");

let instance = await Instance.new_instance();

let scene = instance.create_scene();

const controls = new Float32Array([
  -1.0, 1.0, 0.0, 0.0, 0.5, 0.0, 1.0, 1.0, 0.0,
  -1.0, 0.0, 0.0, 0.0, 0.0, 2.0, 1.0, 0.0, 0.0,
  -1.0, -1.0, 0.0, 0.0, -1.0, 0.0, 1.0, -1.0, 0.0,
])

const empty = new Float32Array(0);

let surface = scene.add_surface(2, 2, controls, 3, 3, empty, empty, empty);

let canvas = document.createElement("canvas");
document.body.appendChild(canvas);

let viewport = instance.create_viewport(canvas);

while (true) {
  instance.draw_scene_to_viewport(scene, viewport);

  scene.rotate_geometry(surface, new Float32Array([0, 0, 0]), new Float32Array([0, 1, 0]), 0.02);
  await new Promise(r => setTimeout(r, 0));

}




