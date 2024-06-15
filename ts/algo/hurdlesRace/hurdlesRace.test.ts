import { suite, test, beforeEach, expect } from "vitest";
import { hurdlesRace } from "./hurdlesRace";
import { hurdlesRaceInputs } from "./hurdlesRaceInputs";

const { firstCase, fourthCase, secondCase, thirdCase } = hurdlesRaceInputs;

const testCase = (
  maximumJumpHeight: number,
  race: number[],
  expected: number
) => {
  const result = hurdlesRace(maximumJumpHeight, race);
  expect(result).to.be.equal(expected);
};

suite("hurdlesRace test suite", () => {
  test("First case", () => {
    const {
      input: { maximumJumpHeight, race },
      expected,
    } = firstCase;
    testCase(maximumJumpHeight, race, expected);
  });
  test("Second case", () => {
    const {
      input: { maximumJumpHeight, race },
      expected,
    } = secondCase;
    testCase(maximumJumpHeight, race, expected);
  });
  test("Third case", () => {
    const {
      input: { maximumJumpHeight, race },
      expected,
    } = thirdCase;
    testCase(maximumJumpHeight, race, expected);
  });
  test("Fourth case", () => {
    const {
      input: { maximumJumpHeight, race },
      expected,
    } = fourthCase;
    testCase(maximumJumpHeight, race, expected);
  });
});
