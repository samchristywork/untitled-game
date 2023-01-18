export function render(x, y) {
  let debug=document.querySelector("#debug");

  let canvas=document.querySelector("#canvas");
  let context=canvas.getContext("2d");

  context.clearRect(0, 0, canvas.width, canvas.height);

  var img = document.getElementById("sprites");
  context.drawImage(img, x, 10);

  let sx=0;
  let sy=0;
  let swidth=32;
  let sheight=32;
  let dx=x;
  let dy=y+64;
  let dwidth=32;
  let dheight=32;
  context.drawImage(img, sx, sy, swidth, sheight, dx, dy, dwidth, dheight);

  context.drawImage(img, sx+32, sy, swidth, sheight, dx, dy+32, dwidth, dheight);

  context.rect(x, 0, 10, 10);
  context.stroke();

  return debug.innerHTML;
}
