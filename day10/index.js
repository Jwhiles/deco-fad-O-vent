const testInput = `89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732`;

const realInput = `snip`

const solution = (input) => {
  const grid = input.split("\n").map((row) => row.split("").map(Number));
  // Map of `y:x` to a set of reachable 9s and a count of reachable 9s
  let results = {};

  const grid_height = grid.length - 1;
  const grid_width = grid[0].length - 1;
  const recurse = (y, x) => {
    if (results[`${y}:${x}`]) {
      return results[`${y}:${x}`];
    }
    const height = grid[y][x];
    if (height === 9) {
      const set = new Set([`${y}:${x}`])
      results[`${y}:${x}`] = { set, count: 1 }
      return { set, count: 1 }
    } else {
      const reachable = [
        [0, 1],
        [1, 0],
        [0, -1],
        [-1, 0],
      ].reduce(({ set, count }, [dy, dx]) => {
        const newY = y + dy;
        const newX = x + dx;
        if (
          newY >= 0 &&
          newY <= grid_height &&
          newX >= 0 &&
          newX <= grid_width
        ) {
          if (grid[newY][newX] === height + 1) {
            const found = recurse(newY, newX);
            return { set: new Set([...set, ...found.set]), count: count + found.count };
          }
        }
        return { set, count };
      }, { set: new Set, count: 0 });
      results[`${y}:${x}`] = reachable;
      return reachable
    }
  };


  let result = { size: 0, count: 0 };
  grid.forEach((row, y) => {
    row.forEach((cell, x) => {
      if (cell === 0) {
        const found = recurse(y, x)
        result = { size: result.size + found.set.size, count: result.count + found.count }
      }
    })
  })
  console.log(result)
};


solution(testInput);
solution(realInput);
