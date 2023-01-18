let canvas=document.querySelector("#canvas");
let debug=document.querySelector("#debug");
console.log(canvas);

var pressed={};
canvas.addEventListener("keydown", function(e) {
  pressed[e.keyCode] = true;
  debug.innerHTML=JSON.stringify(pressed);
});

canvas.addEventListener("keyup", function(e) {
  e = e || window.event;
  delete pressed[e.keyCode];
  debug.innerHTML=JSON.stringify(pressed);
});

canvas.focus();

import init, { status } from "./pkg/untitled_game.js";
init().then(() => {
  status();
});
