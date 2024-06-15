import { suite, test, beforeEach, expect } from "vitest";
import { countingValley } from "./countingValley";

suite("Counting valley", () => {
  test("UDDDUDUU", () => {
    const steps = 8;
    const path = "UDDDUDUU";
    const result = countingValley(steps, path);
    console.log({ result  })
    expect(result).to.be.equal(1);
  });
  test("DDUUDDUDUUUD", () => {
    const steps = 12;
    const path = "DDUUDDUDUUUD";
    const result = countingValley(steps, path);
    console.log({ result  })
    expect(result).to.be.equal(2);
  });
});
