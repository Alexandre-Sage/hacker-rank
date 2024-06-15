#![allow(dead_code)]
pub struct Point {
    x: u16,
    y: u16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

pub fn format_line(input: &str) -> Vec<Point> {
    let mut input: Vec<&str> = input.split("\n").collect();
    input.remove(0);
    input
        .iter()
        .map(|item| {
            let item: Vec<&str> = item.split(" ").collect();
            Point::new(item[0].parse().unwrap(), item[1].parse().unwrap())
        })
        .collect()
}

fn proccess_game(game: &Point) -> &str {
    if game.x % 4 == 0 || game.x % 4 == 3 || game.y % 4 == 0 || game.y % 4 == 3 {
        "First"
    } else {
        "Second"
    }
}

fn chess_board_game(input: &str) -> String {
    let games = format_line(input);
    games
        .iter()
        .map(proccess_game)
        .collect::<Vec<&str>>()
        .join("\n")
}

static FIRST_CASE: &str = "3\n5 2\n5 3\n8 8";
static FIRST_CASE_RESULT: &str = "Second\nFirst\nFirst";
#[test]
fn first_case() {
    let result = chess_board_game(FIRST_CASE);

    assert_eq!(&result, FIRST_CASE_RESULT);
}
