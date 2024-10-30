const rangeFromZero = <T>(end: number, callBack: () => T) =>
  Array(end).fill(0).map(callBack);

class GridCoords {
  private constructor(
    private readonly _x: number,
    private readonly _y: number
  ) {}
  static from_xy = (x: number, y: number) => new this(x, y);

  public getX(): number {
    return this._x;
  }

  public getY(): number {
    return this._y;
  }

  public delta = (gridCoords: GridCoords): [number, number] => [
    this.getX() - gridCoords.getX(),
    this.getY() - gridCoords.getY(),
  ];

  public deltaScore = (gridCoords: GridCoords) => {
    const [x, y] = this.delta(gridCoords);
    return x + y;
  };
}

interface Brain {
  bot: GridCoords;
  dust: GridCoords[];
}

export const ACTIONS = {
  UP: "UP",
  DOWN: "DOWN",
  LEFT: "LEFT",
  RIGHT: "RIGHT",
  CLEAN: "CLEAN",
} as const;

export type Actions = (typeof ACTIONS)[keyof typeof ACTIONS];

const parseInput = (input: string): Brain => {
  const splited = input.split("\n");
  const botStr = splited.shift()?.split(" ") as string[];
  const bot = GridCoords.from_xy(parseInt(botStr[0]), parseInt(botStr[1]));
  const dust = splited
    .flatMap((line, y) =>
      line
        .split("")
        .map((coords, x) => (coords === "d" ? GridCoords.from_xy(x, y) : null))
    )
    .filter((item) => item !== null) as GridCoords[];
  return { bot, dust };
};

const proccessTurn = (
  { bot, dust }: Brain,
  actions: Actions[] = []
): Actions[] => {
  if (!dust.length) return actions;
  const nearest = dust.reduce((acc, cur) =>
    bot.deltaScore(acc) < bot.deltaScore(cur) ? cur : acc
  );
  const [x, y] = bot.delta(nearest);
  const yMoves =
    y !== 0
      ? rangeFromZero(Math.abs(y), () => (y > 0 ? ACTIONS.UP : ACTIONS.DOWN))
      : [];

  const xMoves =
    x !== 0
      ? rangeFromZero(Math.abs(x), () => (x > 0 ? ACTIONS.LEFT : ACTIONS.RIGHT))
      : [];

  if (dust.length === 2) console.log({ bot, dust, xMoves, yMoves });
  return proccessTurn(
    {
      bot: nearest,
      dust: dust.filter((item) => item !== nearest),
    },
    actions.concat(yMoves).concat(xMoves).concat("CLEAN")
  );
};

export const botClean = (input: string) => {
  const brain = parseInput(input);
  console.log(brain.dust);
  return proccessTurn(brain);
};
