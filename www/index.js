import { memory } from "wasm-julia/wasm_julia_bg";
import { ZPlane } from "wasm-julia";

const CELL_SIZE = 3; // px
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

//const COLOURS = ["#FFFFFF", "#E0E0E0", "#C0C0C0", "#A0A0A0", "#808080", "#606060", "#404040", "#202020", "#000000"];

function getColours() {
  var colours = [];
  for (var i=0; i < 256; ++i)
    colours.push("#"+((255-i) * 0x010101).toString(16).toUpperCase());
  return colours;
};

const COLOURS = getColours();


// Construct the z-plane, and get its width and height.
const width = 250;
const height = 250;
const zplane = ZPlane.new(-0.01, 0.0651, 2.0, width, height);

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("julia-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

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

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);

      // ctx.fillStyle = cells[idx] === 0
      //   ? DEAD_COLOR
      //   : ALIVE_COLOR;

      ctx.fillStyle = COLOURS[cells[idx]];

      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
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