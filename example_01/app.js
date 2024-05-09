import { hello_world } from '../engine/pkg'

console.log("running example 1");


var canvas = document.createElement("canvas");
document.body.appendChild(canvas);


hello_world(canvas);

