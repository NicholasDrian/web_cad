import { create_instance, get_from_js } from '../engine/pkg'

console.log("running example 1");


var canvas1 = document.createElement("canvas");
document.body.appendChild(canvas1);
var canvas2 = document.createElement("canvas");
document.body.appendChild(canvas2);

await create_instance([canvas1, canvas2]);

