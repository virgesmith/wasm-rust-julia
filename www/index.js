import { memory } from "wasm-julia/wasm_julia_bg";
import { ZPlane } from "wasm-julia";

const CELL_SIZE = 3; // px

function getColours() {
  var colours = [];
  for (var i=0; i < 256; ++i) {
    var x = Math.round((16 - Math.sqrt(i))*16) - 1;
    colours.push("#"+(x * 0x010101).toString(16).toUpperCase());
    console.log(colours[i]);
  }
  return colours;
};

const COLOURS = getColours();

// Construct the z-plane, and get its width and height.
const width = 320;
const height = 320;
const zplane = ZPlane.new(-0.1, 0.651, 2.0, width, height);

// Give the canvas room for all of our cells and a 1px border
// around each of them.
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
  zplane.tick();
  drawCells();

  animationId = requestAnimationFrame(renderLoop);
};

const isPaused = () => {
  return animationId === null;
};

const drawCells = () => {
  const cellsPtr = zplane.cells();
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

play();