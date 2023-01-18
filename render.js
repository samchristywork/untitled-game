export function render(counter) {
  let canvas=document.querySelector("#canvas");
  let context=canvas.getContext("2d");

  var img = document.getElementById("sprites");
  context.drawImage(img, counter, 10);

  let sx=0;
  let sy=0;
  let swidth=32;
  let sheight=32;
  let dx=counter;
  let dy=64;
  let dwidth=32;
  let dheight=32;
  context.drawImage(img, sx, sy, swidth, sheight, dx, dy, dwidth, dheight);

  context.rect(counter, 0, 10, 10);
  context.stroke();
}
