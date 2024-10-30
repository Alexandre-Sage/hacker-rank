pub fn parse_input(input: &str) -> Vec<i32> {
    let splited = input
        .split("\n")
        .map(|i| i.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    splited.get(1..).unwrap().to_owned()
}
pub fn utopian_tree(line: i32) -> i32 {
    println!("{line}");
    let mut result = 1;
    for i in 1..line + 1 {
        println!("{i}");
        if i % 2 == 0 {
            result += 1
        } else {
            result *= 2
        }
    }
    result
}

pub fn run(input: &str) -> Vec<i32> {
    let input = parse_input(input);
    input.into_iter().map(utopian_tree).collect()
}
static FIRST_INPUT: &str = "3\n0\n1\n4";
static FIRST_RESUT: [i32; 3] = [1, 2, 7];
#[test]
fn first_case() {
    let result = run(FIRST_INPUT);
    assert_eq!(result, FIRST_RESUT)
}
