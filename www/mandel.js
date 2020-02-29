import { memory } from "wasm-julia/wasm_julia_bg";
import { Mandel } from "wasm-julia";
import { getColours } from "./common";

const CELL_SIZE = 1; // px

const DEPTH = 1024;

const COLOURS = getColours(DEPTH);

const canvas = document.getElementById("mandel-canvas");

// Construct the z-plane, and get its width and height.
// If it's resolution does not match change it
if (canvas.width !== canvas.clientWidth || canvas.clientHeight !== height) {
  canvas.width = canvas.clientWidth;
  canvas.height = canvas.clientHeight;
}

const width = canvas.width / CELL_SIZE;
const height = canvas.height / CELL_SIZE;
const mandel = Mandel.new(width, height, DEPTH);

const ctx = canvas.getContext('2d');

const getIndex = (row, column) => {
  return row * width + column;
};

const drawCells = () => {
  const cellsPtr = mandel.cells();
  const cells = new Uint16Array(memory.buffer, cellsPtr, width * height);

  ctx.beginPath();

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);

      if (cells[idx] == 0) continue;

      ctx.fillStyle = COLOURS[cells[idx]];

      ctx.fillRect(
        col * CELL_SIZE,
        row * CELL_SIZE,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }

  ctx.stroke();
};

(function() {
  "use strict";

  document.onmousedown = handleMouseClick;
  function handleMouseClick(event) {
    var dot, eventDoc, doc, body, pageX, pageY;
    
    event = event || window.event; // IE-ism
    const rect = canvas.getBoundingClientRect();  
    
    const x = (event.clientX - rect.left) / CELL_SIZE;
    const y = (event.clientY - rect.top) / CELL_SIZE;

    //console.log(x, y);
    if (x >= 0 && y >= 0 && x <= width && y <= height) {
      //console.log(
      mandel.zoom(x, y);
      drawCells();
    }
  }
})();
  

drawCells();