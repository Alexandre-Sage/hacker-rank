#![allow(dead_code)]

use std::{
    collections::HashMap,
    iter::Map,
    path::PathBuf,
    str::FromStr,
    time::{Duration, Instant},
    usize,
};

fn parse_default_state(grid: &[String]) -> HashMap<(i64, i64), Option<i64>> {
    let grid = grid.into_iter().map(|x| x.chars());
    grid.enumerate()
        .flat_map(|(y, c)| {
            c.enumerate().map(move |(x, c)| {
                let point = (x as i64, y as i64);
                if c == 'O' {
                    (point, Some(3))
                } else {
                    (point, None)
                }
            })
        })
        .collect()
}

fn create_output(state: HashMap<(i64, i64), Option<i64>>) -> Vec<String> {
    let mut sorted = state
        .into_iter()
        .collect::<Vec<((i64, i64), Option<i64>)>>();
    sorted.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    let result: Vec<Vec<String>> = sorted.into_iter().fold(Vec::new(), |mut acc, (pos, bomb)| {
        let fill = bomb.map(|_| "O").unwrap_or(".").to_string();
        if acc.get(pos.1 as usize).is_some() {
            acc[pos.1 as usize].push(fill);
        } else {
            acc.push(vec![fill])
        }
        acc
    });
    result.iter().map(|x| x.concat()).collect()
}

fn bomberman(seconds: i32, grid: &[String]) -> Vec<String> {
    let grid_size_x = grid[0].len();
    let grid_size_y = grid.len();
    let mut default_state = parse_default_state(grid);
    for x in 1..seconds + 1 {
        let mut bomb_to_remove = Vec::new();
        for (pos, bomb) in &mut default_state {
            match bomb {
                Some(mut count) => {
                    let _ = bomb.insert(count - 1);
                    count -= 1;
                    if count == 0 {
                        for i in -1..2 {
                            let x = pos.0 + i;
                            let y = pos.1 + i;
                            if x >= 0 && x < grid_size_x as i64 {
                                //default_state.insert((x,pos.1), None);
                                bomb_to_remove.push((x, pos.1));
                            }
                            if y >= 0 && y < grid_size_y as i64 {
                                bomb_to_remove.push((pos.0, y));
                            }
                        }
                    }
                }
                None => {
                    if x % 2 == 0 {
                        let _ = bomb.insert(3);
                    }
                }
            }
        }
        for pos in bomb_to_remove {
            default_state.insert(pos, None);
        }
        println!("{x}\n{:#?}", create_output(default_state.clone()));
    }

    create_output(default_state)
    //todo!()
}

static FIRST_CASE_INPUT: &[u8] = b".......
...O...
....O..
.......
OO.....
OO.....";

static FIRST_CASE_EXPECT: &[u8] = b"OOO.OOO
OO...OO
OOO...O
..OO.OO
...OOOO
...OOOO";

static TIMED_OUT_CASE_IPT:&str = "OOOO.O.O...OOO.O.O........O.OOO.O.....OO..O..O...OOO....O.OOO....O...O....O..O.O.O.....OOOO.O...O....OO.O...........O.O..O.O..O...OO.OOO......O........O...O....O.O..O....O.......OOOO.O..........OO.O";
static TIMED_OUT__OTP:&str = ".........O.........OOOOOO.........OOO..........O.....OO.......OO...O...OO..........OOO........O...OO......OOOOOOOOO.............O........OOOO...OOOOOO...O...OO........OO...OOOOO........OOOOOOOO.....";
#[test]
fn bomberman_first_case() {
    let itp = String::from_utf8(FIRST_CASE_INPUT.to_vec())
        .unwrap()
        .to_string()
        .lines()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let itp = itp.as_slice();
    let otp = String::from_utf8(FIRST_CASE_EXPECT.to_vec())
        .unwrap()
        .lines()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let result = bomberman(3, itp);
    println!("{:#?}", otp);
    println!("{:#?}", result);
    let ok = result.into_iter().zip(otp).all(|(r, e)| r == e);
    assert!(ok)
}

#[test]
fn bomberman_second_case() {
    let sec = 198;
    let input = std::fs::read_to_string(
        PathBuf::from_str(&format!(
            "{}/src/bomberman/second_case/input.txt",
            std::env::current_dir().unwrap().to_str().unwrap()
        ))
        .unwrap(),
    )
    .unwrap();
    let output = std::fs::read_to_string(
        PathBuf::from_str(&format!(
            "{}/src/bomberman/second_case/output.txt",
            std::env::current_dir().unwrap().to_str().unwrap()
        ))
        .unwrap(),
    )
    .unwrap();
    let input = input
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let output = output
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let res = bomberman(sec, input.as_slice());
    let ok = res.into_iter().zip(output).all(|(r, e)| r == e);
    assert!(ok)
}

#[test]
fn bomberman_third_case() {
    let sec = 6;
    let input = std::fs::read_to_string(
        PathBuf::from_str(&format!(
            "{}/src/bomberman/third_case/input.txt",
            std::env::current_dir().unwrap().to_str().unwrap()
        ))
        .unwrap(),
    )
    .unwrap();
    let output = std::fs::read_to_string(
        PathBuf::from_str(&format!(
            "{}/src/bomberman/third_case/output.txt",
            std::env::current_dir().unwrap().to_str().unwrap()
        ))
        .unwrap(),
    )
    .unwrap();
    let input = input
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let output = output
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let res = bomberman(sec, input.as_slice());
    let ok = res.into_iter().zip(output).all(|(r, e)| r == e);
    assert!(ok)
}

#[test]
fn timed_out_case() {
    let start = Instant::now();
    let sec = 329973043;
    let input = &[TIMED_OUT_CASE_IPT.to_string()];
    let output = vec![TIMED_OUT__OTP];
    let res = bomberman(sec, input);
    let ok = res.into_iter().zip(output).all(|(r, e)| r == e);
    let elapsed = start.elapsed();
    if elapsed > Duration::from_secs(5) {
        panic!("Test took too long: {:?}", elapsed); // Fail if the test took longer than 5 seconds
    }

    assert!(ok)
}
