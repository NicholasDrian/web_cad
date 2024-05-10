import { create_instance, draw, add_mesh, Vec3 } from '../engine/pkg'

console.log("running example 1");


let canvas1 = document.createElement("canvas");
document.body.appendChild(canvas1);
let canvas2 = document.createElement("canvas");
document.body.appendChild(canvas2);

let instance_handle = await create_instance([canvas1, canvas2]);


const vertices = [
  new Vec3(-0.0868241, 0.49240386, 0.0),
  new Vec3(-0.49513406, 0.06958647, 0.0),
  new Vec3(-0.21918549, -0.44939706, 0.0),
  new Vec3(0.35966998, -0.3473291, 0.0),
  new Vec3(0.44147372, 0.2347359, 0.0),
];

const normals = [
  new Vec3(0, 0, 1),
  new Vec3(0, 1, 0),
  new Vec3(1, 0, 0),
  new Vec3(0, 1, 0),
  new Vec3(1, 0, 0),
]

const indices = [0, 1, 4, 1, 2, 4, 2, 3, 4];

add_mesh(instance_handle, vertices, normals, indices);

draw(instance_handle);


