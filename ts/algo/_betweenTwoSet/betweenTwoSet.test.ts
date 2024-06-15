import { suite, test, beforeEach, expect } from "vitest";
import { betweenTwoSet, findGcd } from "./betweenTwoSet";
import { betweenTwoSetInputs } from "./betweenTwoSetInputs";
const testCase = () => {};
suite("betweenTwoSet test suite", () => {
  suite("Functions", () => {
    test("Pgcd", () => {
      const computation = findGcd(48, 18);
      expect(computation).to.be.equal(6);
    });
  });
  test("First case ", () => {
    const { a, b, result } = betweenTwoSetInputs.first;
    const computation = betweenTwoSet(a, b);
    expect(computation).to.be.equal(result);
  });
  test("Second case", () => {
    const { a, b, result } = betweenTwoSetInputs.scond;
    const computation = betweenTwoSet(a, b);
    expect(computation).to.be.equal(result);
  });
  test.skip("Third case", () => {});
});
