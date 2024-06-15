ALGO_DIR="./algo"
rm -rf $ALGO_DIR/$1
function_script="
export const $1 = () => {
 return;
};"
testScript="
 import { suite, test, beforeEach, expect } from 'vitest';
 import { $1 } from './$1';
 import { $1Inputs } from './$1Inputs';

 const testCase = () => {};

 suite('$1 test suite', () => {
  test('First case', () => {

  });
  test.skip('Second case', () => {
   
  });
  test.skip('Third case', () => {
   
  });
 });"
inputs="
 export const $1Inputs = {}
"
mkdir $ALGO_DIR/$1
cd $ALGO_DIR/$1
echo $testScript > $1.test.ts 
echo $function_script > $1.ts
echo $inputs  >  $1Inputs.ts
#touch $1.ts $1.test.ts $1Inputs.ts

