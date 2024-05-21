import { Instance, get_from_rust, print_u32 } from '../engine/pkg'

console.log("running example 1");

let instance = await Instance.new_instance();

let mesh_scene = instance.create_scene();
let polyline_scene = instance.create_scene();
let curve_scene = instance.create_scene();
let surface_scene = instance.create_scene();

const normals = [
  0, 0, -1,
  0, -1, 0,
  1, 0, 0,
  0, 1, 0,
  1, 0, 0,
]

const vertices = [
  -0.0868241, 0.49240386, 0.0,
  -0.49513406, 0.06958647, 0.0,
  -0.21918549, -0.44939706, 0.0,
  0.35966998, -0.3473291, 0.0,
  0.44147372, 0.2347359, 0.0,
];

const indices = [0, 1, 4, 1, 2, 4, 2, 3, 4];

const surface_controls = [
  -1.0, 1.0, 0.0, 0.0, 0.5, 0.0, 1.0, 1.0, 0.0,
  -1.0, 0.0, 0.0, 0.0, 0.0, 2.0, 1.0, 0.0, 0.0,
  -1.0, -1.0, 0.0, 0.0, -1.0, 0.0, 1.0, -1.0, 0.0,
]

let mesh = mesh_scene.add_mesh(vertices, normals, indices);
let poly = polyline_scene.add_polyline(vertices);
let curve = curve_scene.add_curve(2, vertices, [], []);
let surface = surface_scene.add_surface(2, 2, surface_controls, 3, 3, [], [], []);

let canvas1 = document.createElement("canvas");
document.body.appendChild(canvas1);

let canvas2 = document.createElement("canvas");
document.body.appendChild(canvas2);

let canvas3 = document.createElement("canvas");
document.body.appendChild(canvas3);

let canvas4 = document.createElement("canvas");
document.body.appendChild(canvas4);

let mesh_viewport = instance.create_viewport(canvas1);
let polyline_viewport = instance.create_viewport(canvas2);
let curve_viewport = instance.create_viewport(canvas3);
let surface_viewport = instance.create_viewport(canvas4);

while (true) {
  instance.draw_scene_to_viewport(mesh_scene, mesh_viewport);
  instance.draw_scene_to_viewport(polyline_scene, polyline_viewport);
  instance.draw_scene_to_viewport(curve_scene, curve_viewport);
  instance.draw_scene_to_viewport(surface_scene, surface_viewport);

  mesh_scene.rotate_geometry(mesh, [0, 0, 0], [0, 1, 0], 0.02);
  polyline_scene.rotate_geometry(poly, [0, 0, 0], [0, 1, 0], 0.02);
  curve_scene.rotate_geometry(curve, [0, 0, 0], [0, 1, 0], 0.02);
  surface_scene.rotate_geometry(surface, [0, 0, 0], [0, 1, 0], 0.02);

  await new Promise(r => setTimeout(r, 20));
}


