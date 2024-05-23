import { Instance, CameraType } from '../../engine/pkg'

let instance = await Instance.new_instance();

let scene = instance.create_scene();

const size_x = 100;
const size_y = 100;

function random_controls(width: number, height: number): Float32Array {
  let res: Float32Array = new Float32Array(width * height * 3);
  for (let i = 0; i < height; i++) {
    for (let j = 0; j < width; j++) {
      res[3 * (i * width + j)] = j / width * size_x - size_x / 2;
      res[3 * (i * width + j) + 1] = Math.random() * 5;
      res[3 * (i * width + j) + 2] = i / height * size_y - size_y / 2;
    }
  }
  return res;
}


const empty = new Float32Array(0);

let control_count_u = 100;
let control_count_v = 100;


let surface = scene.add_surface(2, 3, random_controls(control_count_u, control_count_v), control_count_u, control_count_v, empty, empty, empty);

let control_count_u_slider: HTMLInputElement = <HTMLInputElement>document.getElementById("control count u");
control_count_u_slider.addEventListener("change", (_) => {
  control_count_u = Number(control_count_u_slider.value)
  scene.delete_geometry(surface);
  surface = scene.add_surface(2, 3, random_controls(control_count_u, control_count_v), control_count_u, control_count_v, empty, empty, empty);
});

let control_count_v_slider: HTMLInputElement = <HTMLInputElement>document.getElementById("control count v");
control_count_v_slider.addEventListener("change", (_) => {
  control_count_v = Number(control_count_v_slider.value)
  scene.delete_geometry(surface);
  surface = scene.add_surface(2, 3, random_controls(control_count_u, control_count_v), control_count_u, control_count_v, empty, empty, empty);
});


let canvas = document.createElement("canvas");
document.body.appendChild(canvas);

let viewport = instance.create_viewport(canvas);

// TODO: shouldnt need to pass aspect
viewport.set_camera_params(new Float32Array([0, 20, -60]), new Float32Array([0, 0, 0]), 1.5, 1.5, 0.001, 100000.0, CameraType.CAD);


while (true) {

  scene.update_surface_params(surface, 3, 3, random_controls(control_count_u, control_count_v), empty, empty, empty);
  instance.draw_scene_to_viewport(scene, viewport);

  // yeild
  await new Promise(r => setTimeout(r, 200));

}

