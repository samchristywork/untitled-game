export function render(counter) {
  let canvas=document.querySelector("#canvas");
  let context=canvas.getContext("2d");
  context.rect(counter, 0, 10, 10);
  context.stroke();
}
