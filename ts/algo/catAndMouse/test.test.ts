import { suite, test, beforeEach, expect } from "vitest";
import { catAndMouse } from "./catAndMouse";
import { catAndMouseInputs, finalInputs } from "./inputs";

const testCase = (
  firstCat: number,
  secondCat: number,
  mouse: number,
  expected: string
) => {
  const result = catAndMouse(firstCat, secondCat, mouse);
  expect(result).to.be.eql(expected);
};

suite("Cat and mouse suite", () => {
  test("First test case", () => {
    const {
      first: { expected, firstCat, mouse, secondCat },
    } = catAndMouseInputs;
    testCase(firstCat, secondCat, mouse, expected);
  });
  test("Second test case", () => {
    const {
      second: { expected, firstCat, mouse, secondCat },
    } = catAndMouseInputs;
    testCase(firstCat, secondCat, mouse, expected);
  });
  test("Third test case", () => {
    const {
      third: { expected, firstCat, mouse, secondCat },
    } = catAndMouseInputs;
    testCase(firstCat, secondCat, mouse, expected);
  });
  finalInputs.forEach(({ expected, firstCat, mouse, secondCat }, index) => {
    test(`${firstCat}, ${secondCat}, ${mouse}, ${expected},  test case number ${index}`, () => {
      testCase(firstCat, secondCat, mouse, expected);
    });
  });
});
