export const findGcd = (x: number, y: number): number => {
  if (x === y) return x;
  return x > y ? findGcd(x - y, y) : findGcd(x, y - x);
};

export const findLcm = (x: number, y: number) =>
  Math.abs(x * y) / findGcd(x, y);


export const betweenTwoSet = (a: number[], b: number[]) => {
  let res: number[] = [];
  let num = 0 
  let lcm = a[0];
  for (const number of a) lcm = findLcm(lcm, number);
  let gcd = b[0];
  for (const number of b) gcd = findGcd(gcd, number);
  let multiple = 0;
  while (multiple <= gcd) {
   multiple += lcm;
   if(gcd % multiple === 0 ){
    num ++
    res.push(multiple)
   }
  }
  return num;
};
