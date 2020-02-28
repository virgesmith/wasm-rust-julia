import { memory } from "wasm-julia/wasm_julia_bg";
import { Mandel } from "wasm-julia";
import { getColours } from "./common";

const CELL_SIZE = 3; // px

const DEPTH = 1024;

const COLOURS = getColours(DEPTH);

// Construct the z-plane, and get its width and height.
const width = 640;
const height = 640;
const mandel = Mandel.new(width, height, DEPTH);

const canvas = document.getElementById("mandel-canvas");
canvas.height = CELL_SIZE * height;
canvas.width = CELL_SIZE * width;

const ctx = canvas.getContext('2d');

const getIndex = (row, column) => {
  return row * width + column;
};

const drawCells = () => {
  const cellsPtr = mandel.cells();
  const cells = new Uint16Array(memory.buffer, cellsPtr, width * height);

  ctx.beginPath();

  ctx.fillStyle = COLOURS[0];
  ctx.fillRect(
    0,
    0,
    CELL_SIZE * width,
    CELL_SIZE * height
  );

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
    
    // If pageX/Y aren't available and clientX/Y
    // are, calculate pageX/Y - logic taken from jQuery
    // Calculate pageX/Y if missing and clientX/Y available
    // if (event.pageX == null && event.clientX != null) {
    //   eventDoc = (event.target && event.target.ownerDocument) || document;
    //   doc = eventDoc.documentElement;
    //   body = eventDoc.body;

    //   event.pageX = event.clientX +
    //     (doc && doc.scrollLeft || body && body.scrollLeft || 0) -
    //     (doc && doc.clientLeft || body && body.clientLeft || 0);
    //   event.pageY = event.clientY +
    //     (doc && doc.scrollTop  || body && body.scrollTop  || 0) -
    //     (doc && doc.clientTop  || body && body.clientTop  || 0 );
    // }

    console.log(mandel.zoom((event.pageX - window.innerWidth/2 + width/2) ,
                  (event.pageY - window.innerHeight/2 + height /2)));
    drawCells();
  }
})();
  

drawCells();