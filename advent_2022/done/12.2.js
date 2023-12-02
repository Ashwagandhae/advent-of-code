const fs = require('fs');
const path = require('path');

// open file
let start_pos_list = [];
let end_pos;
const terrain = fs
  .readFileSync(path.join(__dirname, '../data/12.txt'), 'utf8')
  .split('\n')
  .map((line, y) =>
    line.split('').map((char, x) => {
      if (char === 'S' || char === 'a') {
        start_pos_list.push({ x, y });
        char = 'a';
      }
      if (char === 'E') {
        end_pos = { x, y };
        char = 'z';
      }
      return char.charCodeAt(0) - 97;
    })
  );

function findShortestPath(start_pos) {
  const visited = Array.from({ length: terrain.length }, () =>
    Array.from({ length: terrain[0].length }, () => false)
  );
  let queue = [start_pos];
  let steps = 0;
  while (queue.length) {
    if (queue.some(({ x, y }) => x === end_pos.x && y === end_pos.y)) {
      return steps;
    }
    // pretty print visited
    queue = queue
      .filter(({ x, y }) => {
        if (visited[y][x]) {
          return false;
        }
        visited[y][x] = true;
        return true;
      })
      .map(({ x, y }) =>
        [
          { x: x + 1, y },
          { x: x - 1, y },
          { x, y: y + 1 },
          { x, y: y - 1 },
        ].filter(
          (next) =>
            next.x >= 0 &&
            next.x < terrain[0].length &&
            next.y >= 0 &&
            next.y < terrain.length &&
            terrain[next.y][next.x] !== -1 &&
            terrain[next.y][next.x] <= terrain[y][x] + 1
        )
      )
      .flat();

    steps++;
  }
  return null;
}

// get min steps for each start position, and get min of that
console.log(
  Math.min(
    ...start_pos_list
      .map((start_pos) => findShortestPath(start_pos))
      .filter((len) => len != null)
  )
);
