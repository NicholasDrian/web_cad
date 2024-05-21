import { Instance } from '../../engine/pkg'

console.log("running example 2");

let instance = await Instance.new_instance();

let scene = instance.create_scene();


let random_controls = function(width: number, height: number): Float32Array {
  let res: Float32Array = new Float32Array(width * height * 3);

  for (let i = 0; i < height; i++) {
    for (let j = 0; j < width; j++) {
      res[3 * (i * width + j)] = j;
      res[3 * (i * width + j) + 1] = i;
      res[3 * (i * width + j) + 2] = Math.random() * 2 - 1;
    }
  }




  return res;
}



const empty = new Float32Array(0);

let width = 10;
let height = 10;
let surface = scene.add_surface(2, 3, random_controls(width, height), width, height, empty, empty, empty);

let canvas = document.createElement("canvas");
document.body.appendChild(canvas);

let viewport = instance.create_viewport(canvas);

while (true) {
  scene.update_surface_params(surface, 3, 3, random_controls(width, height), empty, empty, empty);
  instance.draw_scene_to_viewport(scene, viewport);

  // yeild
  await new Promise(r => setTimeout(r, 0));

}




