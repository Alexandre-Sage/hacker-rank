export const hurdlesRace = (maximumJumpHeight: number, race: number[]) => {
  const highestHurdle = race.reduce((acc, cur) => (cur > acc ? cur : acc));
  return highestHurdle > maximumJumpHeight
    ? highestHurdle - maximumJumpHeight
    : 0;
};
