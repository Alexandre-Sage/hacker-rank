const MOVES = {
  LEFT: "LEFT",
  RIGHT: "RIGHT",
  UP: "UP",
  DOWN: "DOWN",
} as const;

interface Point {
  x: number;
  y: number;
  type: "m" | "p";
}

const point = (x: number, y: number, type: "m" | "p"): Point => ({
  x,
  type,
  y,
});

const input = (input: string) => {
  const splited = input.split("\n");
  const [head, ...tail] = splited;
  return tail.map((item) => item.trim().split(""));
};

const rangeFromZero = (end: number, callBack: () => unknown) =>
  Array(end).fill(0).map(callBack);

const parser = (grid: string[][]) => {
  const positions = grid
    .flatMap((row, y) =>
      row.map((col, x) =>
        col === "m"
          ? point(x, y, col)
          : col === "p"
          ? point(x, y, col)
          : undefined
      )
    )
    .filter((item) => !!item) as Point[];
  const bot = (
    positions[0].type === "m" ? positions.shift() : positions.pop()
  ) as Point;
  const princess = positions.shift() as Point;
  const yDelta = bot.y - princess.y;
  const xDelta = bot.x - princess.x;
  const yMoves = rangeFromZero(Math.abs(yDelta), () =>
    yDelta > 0 ? MOVES.UP : MOVES.DOWN
  );
  const xMoves = rangeFromZero(Math.abs(xDelta), () =>
    xDelta > 0 ? MOVES.LEFT : MOVES.RIGHT
  );
  return yMoves.concat(xMoves).join("\n");
};

export const botSaveThePrincess = (inp: string) => {
  const grid = input(inp);
  return parser(grid);
};
