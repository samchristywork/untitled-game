export function render(x, y) {
  let debug=document.querySelector("#debug");

  let canvas=document.querySelector("#canvas");
  let context=canvas.getContext("2d");

  context.clearRect(0, 0, canvas.width, canvas.height);

  var img = document.getElementById("sprites");

  context.save();
  context.translate(x, y);
  context.rotate(Math.PI / 2);
  let sx=16;
  let sy=0;
  let swidth=16;
  let sheight=16;
  let dx=0;
  let dy=0;
  let dwidth=16;
  let dheight=16;
  context.drawImage(img, sx, sy, swidth, sheight, dx, dy, dwidth, dheight);
  context.restore();

  return debug.innerHTML;
}
