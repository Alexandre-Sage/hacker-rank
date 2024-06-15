"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.hurdlesRace = void 0;
const hurdlesRace = (maximumJumpHeight, race) => {
    const highestHurdle = race.reduce((acc, cur) => (cur > acc ? cur : acc));
    return highestHurdle > maximumJumpHeight
        ? highestHurdle - maximumJumpHeight
        : 0;
};
exports.hurdlesRace = hurdlesRace;
