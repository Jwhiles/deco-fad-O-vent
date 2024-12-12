const testInput = "125 17";

const blink = (n) => {
  if (n === 0) {
    return [1];
  }

  let s = String(n);
  if (s.length % 2 === 0) {
    const firstHalf = Number(s.slice(0, s.length / 2));
    const secondHalf = Number(s.slice(-s.length / 2));
    return [firstHalf, secondHalf];
  }

  return [n * 2024];
};

const solve = (input) => {
  let stones = {}; // { [stoneNumber]: count }

  input.split(" ").forEach((n) => {
    stones[n] = 1;
  });

  for (let i = 1; i <= 100000; i++) {
    let newStones = {};
    for (let key in stones) {
      // for the count stones key we run thru it's turn into and iterate each of those in new stones.

      const blinked = blink(Number(key));
      blinked.forEach((n) => {
        if (newStones[n]) {
          newStones[n] += stones[key];
        } else {
          newStones[n] = stones[key];
        }
      });
    }
    stones = newStones;
    let res = 0;
    for (let key in stones) {
      res += stones[key];
    }
  }

  let res = 0;
  for (let key in stones) {
    res += stones[key];
  }
  console.log(res);
};

solve(testInput);
