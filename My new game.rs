let playerSize = 40;
let playerX;
let playerY;
let gridSize = 50;
let speed = 5;
let lives = 3;

function setup() {
  createCanvas(800, 400);
  playerX = width / 2;
  playerY = height / 2;
}

function draw() {
  background(220);
  
  // Draw border
  strokeWeight(2);
  noFill();
  rect(0, 0, width, height);
  
  // Draw player
  fill(0);
  rect(playerX, playerY, playerSize, playerSize);
  
  // Draw lives
  for (let i = 0; i < lives; i++) {
    fill(255);
    rect(20 * i + 10, 10, 10, 10);
  }
  
  // Move player
  if (keyIsDown(UP_ARROW)) {
    playerY -= speed;
  }
  if (keyIsDown(DOWN_ARROW)) {
    playerY += speed;
  }
  if (keyIsDown(LEFT_ARROW)) {
    playerX -= speed;
  }
  if (keyIsDown(RIGHT_ARROW)) {
    playerX += speed;
  }
  
  // Keep player within canvas bounds
  playerX = constrain(playerX, 0, width - playerSize);
  playerY = constrain(playerY, 0, height - playerSize);
}
