"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const vitest_1 = require("vitest");
const hurdlesRace_1 = require("./hurdlesRace");
const hurdlesRaceInputs_1 = require("./hurdlesRaceInputs");
const { firstCase, fourthCase, secondCase, thirdCase } = hurdlesRaceInputs_1.hurdlesRaceInputs;
const testCase = (maximumJumpHeight, race, expected) => {
    const result = (0, hurdlesRace_1.hurdlesRace)(maximumJumpHeight, race);
    (0, vitest_1.expect)(result).to.be.equal(expected);
};
(0, vitest_1.suite)("hurdlesRace test suite", () => {
    (0, vitest_1.test)("First case", () => {
        const { input: { maximumJumpHeight, race }, expected, } = firstCase;
        testCase(maximumJumpHeight, race, expected);
    });
    (0, vitest_1.test)("Second case", () => {
        const { input: { maximumJumpHeight, race }, expected, } = secondCase;
        testCase(maximumJumpHeight, race, expected);
    });
    (0, vitest_1.test)("Third case", () => {
        const { input: { maximumJumpHeight, race }, expected, } = thirdCase;
        testCase(maximumJumpHeight, race, expected);
    });
    (0, vitest_1.test)("Fourth case", () => {
        const { input: { maximumJumpHeight, race }, expected, } = fourthCase;
        testCase(maximumJumpHeight, race, expected);
    });
});
