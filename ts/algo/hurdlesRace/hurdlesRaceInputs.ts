const inputHandling =(input:string)=>{
 const splitedInput = input.trim().split("\n");
 return {
  maximumJumpHeight: parseInt(splitedInput[0].trim().split(" ")[1]),
  race: splitedInput[1].trim().split(" ").map(item => parseInt(item))
 }
} 
export const hurdlesRaceInputs = {
  firstCase: {
    input: inputHandling(`5 4
    1 6 3 5 2`),
    expected: 2,
  },
  secondCase: {
   input: inputHandling(`5 7
   2 5 4 5 2`),
   expected: 0
  },
  thirdCase: {
   input: inputHandling(`100 87
   56 61 6 83 54 85 43 21 63 76 79 39 78 84 71 10 86 58 2 35 71 23 46 21 61 64 54 6 16 36 22 3 37 25 71 66 9 80 51 58 63 84 64 38 66 19 47 16 66 57 73 77 13 22 79 14 5 86 5 28 81 85 59 28 67 53 74 47 50 70 17 61 33 38 52 64 84 51 3 58 68 80 68 38 56 12 69 34 21 19 83 8 84 27 5 39 5 7 44 74`),
   expected: 0
  },
  fourthCase: {
   input: inputHandling(`100 53
   86 4 83 20 6 81 58 59 53 2 54 62 25 35 79 64 27 49 32 95 100 20 58 39 92 30 67 89 58 81 100 66 73 29 75 81 70 55 18 28 7 35 98 52 30 11 69 48 84 54 13 14 15 86 34 82 92 26 8 53 62 57 50 31 61 85 88 5 80 64 90 52 47 43 40 93 69 70 16 43 7 25 99 12 63 99 71 76 55 17 90 43 27 20 42 84 39 96 75 1`),
   expected: 47
  },
};
