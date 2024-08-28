
import { WebCadInstance } from '../../engine/pkg'

let instance = await WebCadInstance.new_instance();


let scene = instance.create_scene();
let arc = scene.add_arc_start_middle_end(
  new Float32Array([0.0, 0.0, 0.0]),
  new Float32Array([0.0, 1.0, 0.0]),
  new Float32Array([1.0, 0.0, 0.0]),
);

let canvas = document.createElement("canvas");
canvas.width = document.body.clientWidth;
canvas.height = document.body.clientHeight;
document.body.appendChild(canvas);
let viewport = instance.create_viewport(canvas);


while (true) {
  instance.draw_scene_to_viewport(scene, viewport);

  scene.rotate_geometry(arc, new Float32Array([0, 0, 0]), new Float32Array([1, 0, 0]), 0.012342);
  scene.rotate_geometry(arc, new Float32Array([0, 0, 0]), new Float32Array([0, 1, 0]), 0.023452);
  scene.rotate_geometry(arc, new Float32Array([0, 0, 0]), new Float32Array([0, 0, 1]), 0.032342);

  // Yeild 
  await new Promise(r => setTimeout(r, 0));
}


