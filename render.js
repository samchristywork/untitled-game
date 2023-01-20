export function render(s) {
  let sprites=JSON.parse(s);

  let debug=document.querySelector("#debug");

  let canvas=document.querySelector("#canvas");
  let context=canvas.getContext("2d");

  context.clearRect(0, 0, canvas.width, canvas.height);

  var img = document.getElementById("sprites");

  for (let sprite of sprites) {
    context.save();
    context.translate(sprite.x, sprite.y);
    context.font = "14px serif";
    if (sprite.show_debug){
      context.fillText(JSON.stringify(sprite), 0, 0);
    }
    context.rotate(sprite.rotation * Math.PI / 2);
    let sx=16 * sprite.idx;
    let sy=0;
    let swidth=16;
    let sheight=16;
    let dx=0;
    let dy=0;
    let dwidth=16;
    let dheight=16;
    context.drawImage(img, sx, sy, swidth, sheight, dx, dy, dwidth, dheight);
    context.restore();
  }

  return debug.innerHTML;
}
