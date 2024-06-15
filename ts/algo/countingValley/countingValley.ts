export const countingValley = (steps: number, path: string) => {
  const pathArray = path.split("");
  let numberOfValleyCrossed: number = 0;
  let seaLevel: number = 0;
  for (const path of pathArray) {
    if (path === "D") {
      seaLevel -= 1;
    } else if (path === "U") {
      seaLevel += 1;
      if (seaLevel === 0) {
        numberOfValleyCrossed += 1;
      }
    }
  }
  return numberOfValleyCrossed;
};
