#![allow(dead_code)]
fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .split("\n")
        .map(|line| {
            line.split(" ")
                .filter_map(|num_s| num_s.parse().ok())
                .collect()
        })
        .collect()
}

pub fn compare_triplets(triplets: Vec<Vec<i32>>) -> Vec<i32> {
    let mut results = vec![0, 0];
    triplets[0]
        .iter()
        .zip(triplets[1].iter())
        .for_each(|(a, b)| {
            if a > b {
                results[0] += 1
            } else if b > a {
                results[1] += 1
            }
        });
    results
}

fn run(input: &str) -> Vec<i32> {
    let input = parse_input(input);
    compare_triplets(input)
}

static FIRST_INPUT: &str = "17 28 30\n99 16 8";
static FIRST_RESULT: [i32; 2] = [2, 1];
static SECOND_INPUT: &str = "5 6 7\n3 6 10";
static SECOND_RESULT: [i32; 2] = [1, 1];

static THIRD_INPUT: &str = "20 20 30\n20 20 50";
static THIRD_RESULT: [i32; 2] = [0, 1];
#[test]
fn test() {
    assert_eq!(run(FIRST_INPUT), FIRST_RESULT.to_owned());
}
#[test]
fn test_two() {
    assert_eq!(run(SECOND_INPUT), SECOND_RESULT.to_owned());
}
#[test]
fn test_three() {
    assert_eq!(run(THIRD_INPUT), THIRD_RESULT.to_owned());
}
