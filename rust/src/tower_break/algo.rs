#![allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct GameSettings {
    pub towers: i64,
    pub height: i64,
}

impl GameSettings {
    fn new(towers: i64, height: i64) -> Self {
        Self { towers, height }
    }
}

fn format_input(input: &str) -> Vec<(i32, i32)> {
    let mut input: Vec<&str> = input.split("\n").collect();
    input.remove(0);
    input
        .iter()
        .map(|line| {
            let line = line.split_whitespace().collect::<Vec<&str>>();
            (
                line[0].parse::<i32>().unwrap(),
                line[1].parse::<i32>().unwrap(),
            )
        })
        .collect()
}

/// https://www.hackerrank.com/challenges/climbing-the-leaderboard/problem?isFullScreen=true
fn tower_break(towers: i32, height: i32) -> i32 {
    if towers % 2 == 0 || height == 1 {
        2
    } else { 
        1
    }
}

#[cfg(test)]
mod test {
    use crate::tower_break::{
        algo::tower_break,
        input::{
            FIRST_CASE_INPUT, FIRST_CASE_RESULT, SECOND_CASE_INPUT, SECOND_CASE_RESULT,
            THIRD_CASE_INPUT, THIRD_CASE_RESULT,
        },
    };

    use super::format_input;

    fn test(input: &str, expected_result: &[i32]) {
        let input = format_input(input);
        if let Some((index, (towers, height))) = input.iter().enumerate().next() {
            let result = tower_break(*towers, *height);
            assert_eq!(result, expected_result[index])
        }
    }
    #[test]
    fn first_case() {
        test(FIRST_CASE_INPUT, &FIRST_CASE_RESULT)
    }
    #[test]
    fn second_case() {
        test(SECOND_CASE_INPUT, &SECOND_CASE_RESULT)
    }
    #[test]
    fn third_case() {
        test(THIRD_CASE_INPUT, &THIRD_CASE_RESULT)
    }
}
