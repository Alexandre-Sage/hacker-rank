import { suite, test, beforeEach, expect } from "vitest";
import { botSaveThePrincess } from "./botSaveThePrincess";
import { botSaveThePrincessInputs } from "./botSaveThePrincessInputs";
const testCase = (input: string, expected: string) => {
  const result = botSaveThePrincess(input);
  expect(result).toEqual(expected);
};
const {
  first: { result, input },
} = botSaveThePrincessInputs;

suite("botSaveThePrincess test suite", () => {
  test("First case", () => {
    testCase(input, result);
  });
  test.skip("Second case", () => {});
  test.skip("Third case", () => {});
});
