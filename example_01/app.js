import { Instance } from '../engine/pkg'

console.log("running example 1");

let instance = await new Instance();

let mesh_scene = instance.create_scene();
let polyline_scene = instance.create_scene();

// Flat list for performance
const normals = [
  0, 0, 1,
  0, 1, 0,
  1, 0, 0,
  0, 1, 0,
  1, 0, 0,
]

// Flat list for performance
const vertices = [
  -0.0868241, 0.49240386, 0.0,
  -0.49513406, 0.06958647, 0.0,
  -0.21918549, -0.44939706, 0.0,
  0.35966998, -0.3473291, 0.0,
  0.44147372, 0.2347359, 0.0,
];

const indices = [0, 1, 4, 1, 2, 4, 2, 3, 4];

mesh_scene.add_mesh(vertices, normals, indices);
polyline_scene.add_polyline(vertices);

let canvas1 = document.createElement("canvas");
document.body.appendChild(canvas1);

let canvas2 = document.createElement("canvas");
document.body.appendChild(canvas2);

let mesh_viewport = instance.create_viewport(canvas1);
let polyline_viewport = instance.create_viewport(canvas2);

instance.draw_scene_to_viewport(mesh_scene, mesh_viewport);
instance.draw_scene_to_viewport(polyline_scene, polyline_viewport);


