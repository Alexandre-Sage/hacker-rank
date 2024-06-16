ALGO_DIR = "./src/"
rm -rf $ALGO_DIR/$1

SCRIPT = "#![allow(deadcode)]
fn $1() -> () {}" 
