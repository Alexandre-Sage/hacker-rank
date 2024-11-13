#![allow(dead_code)]
// https://www.hackerrank.com/challenges/strange-advertising/problem
fn viral_advertising(n: i32) -> i64 {
    let mut result = 2;
    let mut hist = 2;
    for _ in 1..n {
        hist = (((hist * 3) / 2) as f64).floor() as i64;
        result += hist;
    }
    result
}
#[test]
fn first_case() {
    assert_eq!(viral_advertising(3), 9)
}
#[test]
fn second_case() {
    assert_eq!(viral_advertising(4), 15)
}
#[test]
fn strange_adv_third_case() {
    assert_eq!(viral_advertising(49), 1379192761)
}
#[test]
fn strange_adv_fourth_case() {
    assert_eq!(viral_advertising(50), 2068789129)
}
