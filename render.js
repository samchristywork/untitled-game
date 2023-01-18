export function render(counter) {
  let canvas=document.querySelector("#canvas");
  let context=canvas.getContext("2d");

  var img = document.getElementById("sprites");
  context.drawImage(img, counter, 10);

  context.rect(counter, 0, 10, 10);
  context.stroke();
}
