import { memory } from "wasm-julia/wasm_julia_bg";
import { Julia } from "wasm-julia";
import { getGreyscale } from "./common";

const CELL_SIZE = 3; // px

const COLOURS = getGreyscale();

// Construct the z-plane, and get its width and height.
const width = 320;
const height = 320;
const scale = 2.0; // i.e. [-2, +2]
const julia = Julia.new(-0.1, 0.651, scale, width, height);

const canvas = document.getElementById("julia-canvas");
canvas.height = CELL_SIZE * height;
canvas.width = CELL_SIZE * width;

const ctx = canvas.getContext('2d');

const getIndex = (row, column) => {
  return row * width + column;
};

let animationId = null;

// This function is the same as before, except the
// result of `requestAnimationFrame` is assigned to
// `animationId`.
const renderLoop = () => {
  julia.tick();
  drawCells();

  animationId = requestAnimationFrame(renderLoop);
};

const isPaused = () => {
  return animationId === null;
};

const drawCells = () => {
  const cellsPtr = julia.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

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

  // plot the locus
  ctx.fillStyle = "#FF0000";
  const idx = getIndex(julia.locus_r(), julia.locus_i());
  ctx.fillRect(
    julia.locus_r() * CELL_SIZE-1,
    julia.locus_i() * CELL_SIZE-1,
    CELL_SIZE+1,
    CELL_SIZE+1
  );


  ctx.stroke();
};


const playPauseButton = document.getElementById("play-pause");

const play = () => {
  playPauseButton.textContent = "⏸";
  renderLoop();
};

const pause = () => {
  playPauseButton.textContent = "▶";
  cancelAnimationFrame(animationId);
  animationId = null;
};

playPauseButton.addEventListener("click", event => {
  if (isPaused()) {
    play();
  } else {
    pause();
  }
});

(function() {
  "use strict";

  document.onmousemove = handleMouseMove;
  function handleMouseMove(event) {
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

    julia.set_attract_r(scale * (event.pageX - window.innerWidth/2) / width)
    julia.set_attract_i(scale * (event.pageY - window.innerHeight/2) / width)
    //console.log(scale * (event.pageX - window.innerWidth/2) / width, scale * (event.pageY - window.innerHeight/2) / width);
  }
})();

play();