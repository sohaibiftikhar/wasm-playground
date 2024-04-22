import {Universe} from 'wasm-playground';

const CELL_SIZE = 2; // px
const GRID_COLOR = '#CCCCCC';
const DEAD_COLOR = '#FFFFFF';
const ALIVE_COLOR = '#000000';
const HEIGHT = 256;
const WIDTH = HEIGHT;
const BITS_PER_BYTE = 8;

// setup the universe
const universe = Universe.new(HEIGHT, WIDTH);
const width = universe.width();
const height = universe.height();

// set up the canvas
const canvas = document.getElementById('game-of-life-canvas');
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;
const ctx = canvas.getContext('2d');

// Last animation frame id. If null implies that game is paused.
let animationId = null;
const playPauseButton = document.getElementById('play-pause');
const speedButton = document.getElementById('speed');
const resetButton = document.getElementById('reset');
const randomizeButton = document.getElementById('randomize');

const fps = new class {
  /**
   * Construct an fps timer.
   */
  constructor() {
    this.fps = document.getElementById('fps');
    this.frames = [];
    this.lastFrameTimeStamp = performance.now();
  }

  /**
   * Render the frames per second.
   * @return {undefined}
   **/
  render() {
    // Convert the delta time since the last frame render into a measure
    // of frames per second.
    const now = performance.now();
    const delta = now - this.lastFrameTimeStamp;
    this.lastFrameTimeStamp = now;
    const fps = 1 / delta * 1000;

    // Save only the latest 100 timings.
    this.frames.push(fps);
    if (this.frames.length > 100) {
      this.frames.shift();
    }

    // Find the max, min, and mean of our 100 latest timings.
    let min = Infinity;
    let max = -Infinity;
    let sum = 0;
    for (let i = 0; i < this.frames.length; i++) {
      sum += this.frames[i];
      min = Math.min(this.frames[i], min);
      max = Math.max(this.frames[i], max);
    }
    const mean = sum / this.frames.length;

    // Render the statistics.
    this.fps.textContent = `
Frames per Second:
         latest = ${Math.round(fps)}
avg of last 100 = ${Math.round(mean)}
min of last 100 = ${Math.round(min)}
max of last 100 = ${Math.round(max)}
`.trim();
  }
};

const isPaused = () => {
  return animationId === null;
};

// / Toggle the play/pause state of the game from pause to play.
const play = () => {
  playPauseButton.textContent = '⏸';
  renderLoop();
};

// / Toggle the play/pause state of the game from play to pause.
const pause = () => {
  playPauseButton.textContent = '▶';
  if (!isPaused()) {
    cancelAnimationFrame(animationId);
  }
  animationId = null;
};

const renderLoop = () => {
  fps.render();

  const numTicksPerFrame = speedButton.value;
  universe.tick_n(numTicksPerFrame);

  drawGrid();
  drawCells();

  animationId = requestAnimationFrame(renderLoop);
};

const drawGrid = () => {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  // vertical lines
  for (let i = 0; i <= width; ++i) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
  }
  // horizontal lines
  for (let j = 0; j <= width; ++j) {
    ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
  }
  ctx.stroke();
};

const getIndex = (row, column) => {
  return row * width + column;
};

/* Helper function to check if a bit is set
 * idx: the index of the array of bytes.
 * arr: the array of bytes
 * */
const bitIsSet = (idx, arr) => {
  const byte = Math.floor(idx / BITS_PER_BYTE); // byte index
  // Create a bit mask to check if the bit is set.
  const mask = 1 << (idx % BITS_PER_BYTE);
  return (arr[byte] & mask) === mask;
};

const drawCells = () => {
  const cells = universe.cells();
  ctx.beginPath();
  for (let row = 0; row < height; ++row) {
    for (let col = 0; col < width; ++col) {
      const idx = getIndex(row, col);
      ctx.fillStyle = bitIsSet(idx, cells) ? ALIVE_COLOR : DEAD_COLOR;
      ctx.fillRect(
          col * (CELL_SIZE + 1) + 1,
          row * (CELL_SIZE + 1) + 1,
          CELL_SIZE,
          CELL_SIZE,
      );
    }
  }
  ctx.stroke();
};

const main = () => {
  // Add Event listeners
  playPauseButton.addEventListener('click', () => {
    if (isPaused()) {
      play();
    } else {
      pause();
    }
  });
  canvas.addEventListener('click', (event) => {
    if (event.ctrlKey) {
      console.log('ctrl key pressed');
    }
    const boundingRect = canvas.getBoundingClientRect();
    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;
    const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
    const canvasTop = (event.clientY - boundingRect.top) * scaleY;
    const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
    const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);
    universe.toggle_cell(row, col);
    if (isPaused()) {
      drawGrid();
      drawCells();
    }
  });

  resetButton.addEventListener('click', () => {
    universe.reset();
  });

  randomizeButton.addEventListener('click', () => {
    universe.randomize();
  });

  // Draw the grid and cells
  drawGrid();
  drawCells();
  // start the first loop
  play();
};

main();
