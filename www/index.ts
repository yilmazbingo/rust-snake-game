// wasm-bindgen package exports default module with init(). check pkg/snake_game.js
import init, { World, Direction, GameStatus } from "snake_game";
import { rnd } from "./utils/rnd";

init().then((wasm) => {
  const CELL_SIZE = 20; // this is inpx
  const WORLD_WIDTH = 4;
  const snakeSpawnIdx = rnd(WORLD_WIDTH * WORLD_WIDTH);

  const world = World.new(WORLD_WIDTH, snakeSpawnIdx);
  const worldWidth = world.width();

  const gameControlBtn = document.getElementById("game-control-btn");
  const gameStatus = document.getElementById("game-status");
  const points = document.getElementById("points");

  gameControlBtn.addEventListener("click", (e) => {
    const status = world.game_status();
    if (status === undefined) {
      gameControlBtn.textContent = "Playing...";
      world.start_game();
      play();
    } else {
      location.reload();
    }
  });
  const canvas = <HTMLCanvasElement>document.getElementById("snake-canvas");
  const ctx = canvas.getContext("2d");
  canvas.height = worldWidth * CELL_SIZE;
  canvas.width = worldWidth * CELL_SIZE;
  const snakeCellPtr = world.snake_cells();
  const snakeLen = world.snake_length();
  // array of 4 bytes of each item
  // extracting from memory buffer, address of snakeCellPtr, till snakeLen
  const snakeCells = new Uint32Array(
    wasm.memory.buffer,
    snakeCellPtr,
    snakeLen
  );

  console.log("snake cells address transfer from wasm", snakeCells);

  document.addEventListener("keydown", (event) => {
    // console.log("event key code", event.code);
    // IntlBackslash  for xmodmap to set
    switch (event.code) {
      case "ArrowUp":
        world.change_snake_dir(Direction.Up);
        break;
      case "ArrowRight":
        world.change_snake_dir(Direction.Right);
        break;
      case "ArrowDown":
        world.change_snake_dir(Direction.Down);
        break;
      case "ArrowLeft":
        world.change_snake_dir(Direction.Left);
        break;
    }
  });

  function drawWorld() {
    ctx.beginPath();
    // first draw rows
    for (let x = 0; x < worldWidth + 1; x++) {
      ctx.moveTo(CELL_SIZE * x, 0);
      // this will draw the line
      ctx.lineTo(CELL_SIZE * x, worldWidth * CELL_SIZE);
      // ctx.fillStyle = "#FF0000";
    }
    for (let y = 0; y < worldWidth + 1; y++) {
      ctx.moveTo(0, CELL_SIZE * y);
      ctx.lineTo(worldWidth * CELL_SIZE, CELL_SIZE * y);
    }

    ctx.stroke();
  }

  function drawReward() {
    const idx = world.reward_cell();
    const col = idx % worldWidth;
    const row = Math.floor(idx / worldWidth);

    ctx.beginPath();
    ctx.fillStyle = "#FF0000";
    ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
    ctx.stroke();
  }

  function drawSnake() {
    const snakeCells = new Uint32Array(
      wasm.memory.buffer,
      world.snake_cells(),
      world.snake_length()
    );

    console.log(snakeCells);
    snakeCells
      //cellIdx is the current number inisde the array
      // when snake is crashed, it was overlapping and snake head was under. Now in case I removing the cell from the body
      .filter((cellIdx, i) => !(i > 0 && cellIdx === snakeCells[0]))
      // instead of filter we also could use `reverse()`.reverse mutates the array. so we have to slice the array first
      // then,   ctx.fillStyle = idx === snakeCells.length-1 ? "#78782b" : "#000000";
      // .slice()
      // .reverse()
      .forEach((cellIdx, idx) => {
        const col = cellIdx % worldWidth;
        const row = Math.floor(cellIdx / worldWidth);
        ctx.fillStyle = idx === 0 ? "#78782b" : "#000000";
        ctx.beginPath();
        ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);

        ctx.stroke();
        if (idx === 1000) {
          alert("you won");
        }
      });
  }

  function drawGameStatus() {
    gameStatus.textContent = world.game_status_text();
    points.textContent = world.points().toString();
  }
  function paint() {
    drawWorld();
    drawSnake();
    drawReward();
    drawGameStatus();
  }

  function play() {
    const status = world.game_status();
    if (status == GameStatus.Won || status == GameStatus.Lost) {
      gameControlBtn.textContent = "Replay";
      return;
    }
    const fps = 5;
    setTimeout(() => {
      // using entire canvas width and height to be cleared out
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      // first update the world and then make change
      world.step();
      // redraw the world and snake
      paint();
      // So far, I would update the current position, then i will request the next animation frame.
      // its callback will be called exactly before browser repaints
      requestAnimationFrame(play);
    }, 1000 / fps);
  }
  paint();
});
