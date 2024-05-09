import { create_instance } from '../engine/pkg'

console.log("running example 1");


var canvas1 = document.createElement("canvas");
document.body.appendChild(canvas1);
var canvas2 = document.createElement("canvas");
document.body.appendChild(canvas2);


create_instance([canvas1, canvas2]).await;

