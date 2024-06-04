import { WebCadInstance } from '../../engine/pkg'

let instance = await WebCadInstance.new_instance();


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



let mesh_scene = instance.create_scene();
let mesh = mesh_scene.add_mesh(vertices, normals, indices);
let canvas1 = document.createElement("canvas");
document.body.appendChild(canvas1);
let mesh_viewport = instance.create_viewport(canvas1);

let polyline_scene = instance.create_scene();
let poly = polyline_scene.add_polyline(vertices);
let canvas2 = document.createElement("canvas");
document.body.appendChild(canvas2);
let polyline_viewport = instance.create_viewport(canvas2);

let curve_scene = instance.create_scene();
let curve = curve_scene.add_curve(2, vertices, [], []);
let canvas3 = document.createElement("canvas");
document.body.appendChild(canvas3);
let curve_viewport = instance.create_viewport(canvas3);

let surface_scene = instance.create_scene();
let surface = await surface_scene.add_surface(2, 2, surface_controls, 3, 3, [], [], []);
let canvas4 = document.createElement("canvas");
document.body.appendChild(canvas4);
let surface_viewport = instance.create_viewport(canvas4);

let lines_scene = instance.create_scene();
let lines = lines_scene.add_lines(vertices, new Uint32Array([0, 1, 2, 3, 4, 0, 0, 2, 1, 3, 2, 4, 3, 0]));
let canvas5 = document.createElement("canvas");
document.body.appendChild(canvas5);
let lines_viewport = instance.create_viewport(canvas5);

while (true) {
  instance.draw_scene_to_viewport(mesh_scene, mesh_viewport);
  instance.draw_scene_to_viewport(polyline_scene, polyline_viewport);
  instance.draw_scene_to_viewport(curve_scene, curve_viewport);
  instance.draw_scene_to_viewport(surface_scene, surface_viewport);
  instance.draw_scene_to_viewport(lines_scene, lines_viewport);

  mesh_scene.rotate_geometry(mesh, [0, 0, 0], [0, 1, 0], 0.02);
  polyline_scene.rotate_geometry(poly, [0, 0, 0], [0, 1, 0], 0.02);
  curve_scene.rotate_geometry(curve, [0, 0, 0], [0, 1, 0], 0.02);
  surface_scene.rotate_geometry(surface, [0, 0, 0], [0, 1, 0], 0.02);
  lines_scene.rotate_geometry(lines, [0, 0, 0], [0, 1, 0], 0.02);

  // Yeild 
  await new Promise(r => setTimeout(r, 0));
}


