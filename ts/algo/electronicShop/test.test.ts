import { suite, test, beforeEach, expect } from "vitest";
import { electronicShop } from "./electronicShop";
import { testInputs } from "./input";

suite("Eelectronic Shop", () => {
  test("First case", () => {
    const keyboards = [3, 1],
      drives = [5, 2, 8],
      budget = 10;
    const result = electronicShop(keyboards, drives, budget);
    console.log({ result });
    expect(result).to.be.equal(9);
  });
  test("Second case", () => {
    const keyboards = [4],
      drives = [5],
      budget = 5;
    const result = electronicShop(keyboards, drives, budget);
    console.log({ result });
    expect(result).to.be.equal(-1);
  });
  test("Third test case", () => {
    const {
      third: { budget, drivesString, keyboardsString },
    } = testInputs;
    const keyboards = keyboardsString.split(" ").map((item) => parseInt(item)),
      drives = drivesString.split(" ").map((item) => parseInt(item));
    const result = electronicShop(keyboards, drives, budget);
    console.log({ result });
    expect(result).to.be.equal(budget);
  });
});
