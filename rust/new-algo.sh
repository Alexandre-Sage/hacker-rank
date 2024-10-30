ALGO_DIR = "./src/"
#rm -rf $ALGO_DIR/$1

SCRIPT = "#![allow(deadcode)]
fn $1(input: &str) -> () {}


static FIRST_INPUT:&str = "";
static FIRST_RESULT:unknown = "";
#[test]
fn first_case(){
	let result = $1(FIRST_INPUT);
	assert_eq!(result, FIRST_RESULT);
}" 

mkdir $ALGO_DIR
echo $SCRIPT > "$ALGO_DIR/$1/mod.rs"
