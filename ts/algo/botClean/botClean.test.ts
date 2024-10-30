import { suite, test, beforeEach, expect } from "vitest";
import { botClean } from "./botClean";
import { botCleanInputs } from "./botCleanInputs";

const { first } = botCleanInputs;

const testCase = () => {};
suite("botClean test suite", () => {
  test("First case", () => {
    const { answer, input } = first;
    const result = botClean(input);
    console.log(result);
    expect(result.length).toEqual(answer.length);
  });
  test.skip("Second case", () => {});
  test.skip("Third case", () => {});
});
