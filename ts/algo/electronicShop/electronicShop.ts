export const electronicShop = (
  keyboards: number[],
  drives: number[],
  budget: number
) => {
  let res = -1;
  keyboards.forEach((keyboard) => {
    drives.forEach((drive) => {
      const price = keyboard + drive;
      if (price <= budget && price > res) res = keyboard + drive;
    });
  });
  return res;
};
