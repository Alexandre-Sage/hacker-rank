const ACTIONS = {
  UP: "UP",
  DOWN: "DOWN",
  LEFT: "LEFT",
  RIGHT: "RIGHT",
  CLEAN: "CLEAN",
} as const;

type Actions = (typeof ACTIONS)[keyof typeof ACTIONS];

export const botCleanInputs = {
  first: {
    input: "0 0\n-d--b\n-d---\n---d-\n---d-\n--d-d",
    answer: [
      ACTIONS.RIGHT,
      ACTIONS.CLEAN,
      ACTIONS.DOWN,
      ACTIONS.CLEAN,
      ACTIONS.DOWN,
      ACTIONS.RIGHT,
      ACTIONS.RIGHT,
      ACTIONS.CLEAN,
      ACTIONS.RIGHT,
      ACTIONS.CLEAN,
      ACTIONS.LEFT,
      ACTIONS.DOWN,
      ACTIONS.CLEAN,
      ACTIONS.RIGHT,
      ACTIONS.RIGHT,
      ACTIONS.CLEAN,
    ] as Actions[],
  },
};
