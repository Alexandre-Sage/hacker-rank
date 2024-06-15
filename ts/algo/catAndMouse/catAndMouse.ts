export const catAndMouse = (
  firstCat: number,
  secondCat: number,
  mouse: number,
) => {
  const firstCatDistance = Math.abs(mouse - firstCat);
  const secondCatDistance = Math.abs(mouse - secondCat);
  if (firstCatDistance < secondCatDistance) return "Cat A";
  else if (secondCatDistance < firstCatDistance) return "Cat B";
  else return "Mouse C";
};
