export function render(s, t) {
  let sprites=JSON.parse(s);

  let text=JSON.parse(t);

  let debug=document.querySelector("#debug");

  let canvas=document.querySelector("#canvas");
  let context=canvas.getContext("2d");

  context.imageSmoothingEnabled = false;
  context.clearRect(0, 0, canvas.width, canvas.height);

  var background = document.getElementById("sand");
  context.drawImage(background, 0, 0, 250, 150, 0, 0, canvas.width, canvas.height);

  var img = document.getElementById("sprites");

  for (let t of text) {
    context.fillText(t.text, t.x, t.y);
  }

  for (let sprite of sprites) {
    context.save();
    context.translate(sprite.x, sprite.y);
    context.scale(sprite.scale, sprite.scale);

    if (sprite.show_debug){
      context.fillText(JSON.stringify(sprite), 0, 0);
    }

    context.rotate(sprite.rotation * Math.PI / 2);
    let sx=16 * (sprite.idx % 32);
    let sy=16 * Math.floor(sprite.idx / 32);
    let swidth=sprite.size;
    let sheight=sprite.size;
    let dx=0;
    let dy=0;
    let dwidth=sprite.size;
    let dheight=sprite.size;

    if (sprite.flip) {
      context.scale(-1, 1);
      context.translate(-sprite.size, 0);
    }

    if (sprite.invisible) {
      context.globalAlpha = 0.2;
    }

    context.drawImage(img, sx, sy, swidth, sheight, dx, dy, dwidth, dheight);

    if (sprite.invisible) {
      context.globalAlpha = 1;
    }
    context.restore();
  }

  return debug.innerHTML;
}
