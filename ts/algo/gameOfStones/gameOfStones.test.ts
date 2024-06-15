import { suite, test, beforeEach, expect } from "vitest";
import { gameOfStones } from "./gameOfStones";
import { gameOfStonesInputs } from "./gameOfStonesInputs";
const testCase = (
  input: (typeof gameOfStonesInputs)[keyof typeof gameOfStonesInputs]
) => {
  const result = gameOfStones(input.stones);
  expect(result).to.be.equal(input.result);
};
suite("gameOfStones test suite", () => {
  test("First case", () => {
    testCase(gameOfStonesInputs.firstCase);
  });
  test("Second case", () => {
    testCase(gameOfStonesInputs.secondCase);
  });
  test("Third case", () => {
    testCase(gameOfStonesInputs.thirdCase);
  });
  test("Fourth case", () => {
    testCase(gameOfStonesInputs.fourthCase);
  });
});
