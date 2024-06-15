export const gameOfStones = (stone: number) => stone % 7 === 0 || stone % 7 === 1 ? "Second" : "First"
