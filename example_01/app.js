import * as engine from '../engine/pkg'

console.log("running example 1");

let instance = await engine.create_instance();

let mesh_scene = engine.add_scene(instance);

let polyline_scene = engine.add_scene(instance);

const normals = [
  0, 0, 1,
  0, 1, 0,
  1, 0, 0,
  0, 1, 0,
  1, 0, 0,
]

// TODO: shouldn't need this duplication
// TODO: make instance object thingy
const vertices = [
  -0.0868241, 0.49240386, 0.0,
  -0.49513406, 0.06958647, 0.0,
  -0.21918549, -0.44939706, 0.0,
  0.35966998, -0.3473291, 0.0,
  0.44147372, 0.2347359, 0.0,
];

const indices = [0, 1, 4, 1, 2, 4, 2, 3, 4];

engine.add_mesh(instance, mesh_scene, vertices, normals, indices);
engine.add_polyline(instance, polyline_scene, vertices);

let canvas1 = document.createElement("canvas");
document.body.appendChild(canvas1);

let canvas2 = document.createElement("canvas");
document.body.appendChild(canvas2);

let mesh_viewport = engine.add_viewport(instance, canvas1);
let polyline_viewport = engine.add_viewport(instance, canvas2);

engine.draw_scene_to_viewport(instance, mesh_scene, mesh_viewport);
engine.draw_scene_to_viewport(instance, polyline_scene, polyline_viewport);


