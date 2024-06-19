import { WebCadInstance, CameraType, get_samples_per_segment } from '../../engine/pkg'

import { Queue } from './queue';

let instance = await WebCadInstance.new_instance();

let scene = instance.create_scene();

const size_x = 100;
const size_y = 100;

function random_controls(width: number, height: number): Float32Array {
  let res: Float32Array = new Float32Array(width * height * 3);
  for (let i = 0; i < height; i++) {
    for (let j = 0; j < width; j++) {
      res[3 * (i * width + j)] = j / width * size_x - size_x / 2;
      res[3 * (i * width + j) + 1] = Math.random() * 30;
      res[3 * (i * width + j) + 2] = i / height * size_y - size_y / 2;
    }
  }
  return res;
}

function numberWithCommas(x: number) {
  return x.toString().replace(/\B(?=(\d{3})+(?!\d))/g, ",");
}

function update_stats() {
  let fps: HTMLElement = document.getElementById("fps");
  let surface_vertex_count = document.getElementById("surface vertex count");
  let samples_this_second_display = document.getElementById("samples this second");
  let control_point_count = document.getElementById("control point count");
  let total_samples_display = document.getElementById("total samples");

  // I feel like this function should be in a namespace
  let sps = get_samples_per_segment();

  let sample_count = (control_count_u - 1) * (control_count_v - 1) * sps * sps;
  total_samples += sample_count;
  fps.innerHTML = "FPS: " + samples_this_second_queue.get_size().toString();
  surface_vertex_count.innerHTML = "Surface vertex count: " + numberWithCommas(sample_count);
  control_point_count.innerHTML = "Control point count: " + numberWithCommas(control_count_u * control_count_v);
  total_samples_display.innerHTML = "Total samples: " + numberWithCommas(total_samples);


  samples_this_second += sample_count;
  samples_this_second_queue.push([sample_count, Date.now()]);
  while (Date.now() - samples_this_second_queue.peek()[1] > 1000) {
    samples_this_second -= samples_this_second_queue.pop()[0];
  }

  samples_this_second_display.innerHTML = "Samples this second: " + numberWithCommas(samples_this_second);

}

const empty = new Float32Array(0);
// TODO: update this to load html value
let control_count_u = 15;
let control_count_v = 15;
let degree_u = 2;
let degree_v = 2;
let total_samples = 0;
let samples_this_second = 0;
let samples_this_second_queue = new Queue<[number, number]>();

let surface = await scene.add_surface(degree_u, degree_v, random_controls(control_count_u, control_count_v), control_count_u, control_count_v, empty, empty, empty, true);
let debug_lines = await scene.add_surface_bbh_debug_lines(surface);

let control_count_u_slider = <HTMLInputElement>document.getElementById("control count u");
control_count_u_slider.addEventListener("change", async (_) => {
  control_count_u = Number(control_count_u_slider.value)
  console.log("here");
  await update_surface();
});

let control_count_v_slider = <HTMLInputElement>document.getElementById("control count v");
control_count_v_slider.addEventListener("change", async (_) => {
  control_count_v = Number(control_count_v_slider.value)
  await update_surface();
});

// TODO: prevent invalid degree
let degree_u_slider = <HTMLInputElement>document.getElementById("degree u");
degree_u_slider.addEventListener("change", async (_) => {
  degree_u = Number(degree_u_slider.value)
  await update_surface();
});

let degree_v_slider = <HTMLInputElement>document.getElementById("degree v");
degree_v_slider.addEventListener("change", async (_) => {
  degree_v = Number(degree_v_slider.value)
  await update_surface();
});


let canvas = document.createElement("canvas");
canvas.width = window.innerWidth;
canvas.height = window.innerHeight;
document.body.appendChild(canvas);

let viewport = instance.create_viewport(canvas);

// TODO: shouldnt need to pass aspect
viewport.set_camera_params(new Float32Array([0, 40, -60]), new Float32Array([0, 0, 0]), 1.5, 1.5, 0.001, 100000.0, CameraType.CAD);

async function update_surface() {
  scene.delete_geometry(surface);
  scene.delete_geometry(debug_lines);
  surface = await scene.add_surface(degree_u, degree_v, random_controls(control_count_u, control_count_v), control_count_u, control_count_v, empty, empty, empty, true);
  debug_lines = await scene.add_surface_bbh_debug_lines(surface);
}

while (true) {

  update_stats();
  await update_surface();

  instance.draw_scene_to_viewport(scene, viewport);


  // yeild
  await new Promise(r => setTimeout(r, 20));

}

