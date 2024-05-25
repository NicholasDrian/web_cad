
import { WebCadInstance } from '../../engine/pkg'

let instance = await WebCadInstance.new_instance();


let scene = instance.create_scene();
let arc = scene.add_arc_start_middle_end(
  new Float32Array([0.0, 0.0, 0.0]),
  new Float32Array([0.0, 1.0, 0.0]),
  new Float32Array([1.0, 0.0, 0.0]),
);

let canvas3 = document.createElement("canvas");
document.body.appendChild(canvas3);
let viewport = instance.create_viewport(canvas3);


while (true) {
  instance.draw_scene_to_viewport(scene, viewport);

  scene.rotate_geometry(arc, new Float32Array([0, 0, 0]), new Float32Array([0, 1, 0]), 0.02);

  // Yeild 
  await new Promise(r => setTimeout(r, 0));
}


