#![allow(dead_code)]
use std::collections::{hash_map::Entry, HashMap};

pub fn ranker(hash_map: &mut HashMap<i32, i32>, ranked_score: i32) {
    match hash_map.entry(ranked_score) {
        Entry::Vacant(_value) => {
            let higher_rank = hash_map
                .iter()
                .filter(|(key, _rank)| key > &&ranked_score)
                .min();
            if let Some((_key, rank)) = higher_rank {
                hash_map.insert(ranked_score, rank + 1);
            } else {
                for (_, rank) in hash_map.iter_mut() {
                    *rank += 1
                }
                hash_map.insert(ranked_score, 1);
            };
        }
        _ => (),
    }
}

pub fn rank_array(ranked: &[i32]) -> HashMap<i32, i32> {
    let mut hash_map = HashMap::<i32, i32>::new();
    for ranked_score in ranked {
        ranker(&mut hash_map, *ranked_score);
    }
    return hash_map;
}

pub fn climbing_leaderboard(ranked: &[i32], players: &[i32]) -> Vec<i32> {
    let mut ranked = rank_array(ranked);
    let mut player_ranks: Vec<i32> = Vec::new();
    let mut high_score: i32 = ranked.iter().map(|(key, _)| *key).max().unwrap();
    players.iter().for_each(|player_score| {
        if player_score > &high_score {
            if player_ranks.contains(&1) && player_score != &high_score {
                player_ranks.iter_mut().for_each(|rank| *rank += 1);
                player_ranks.push(1);
            } else {
                player_ranks.push(1);
                ranked.iter_mut().for_each(|(_, val)| *val += 1);
                high_score = *player_score;
            }
            player_ranks.sort();
        } else {
            if let Entry::Occupied(r) = ranked.entry(*player_score) {
                player_ranks.push(*r.get())
            } else {
                let closest_ranked = ranked
                    .iter()
                    .filter(|(key, _)| key > &player_score)
                    .min()
                    .unwrap()
                    .1;
                player_ranks.push(closest_ranked + 1)
            };
            player_ranks.sort();
        }
    });
    player_ranks.sort();
    player_ranks.iter().rev().map(|item| *item).collect()
}

#[cfg(test)]
mod test {
    use crate::climbing_leaderboard::{
        algo::climbing_leaderboard,
        input::{THIRD_CASE_INPUT, THIRD_CASE_RESULT},
    };

    const FIRST_CASE_INPUT: &str = "7\n100 100 50 40 40 20 10\n4\n5 25 50 120";
    const SECOND_CASE_INPUT: &str = "6\n100 90 90 80 75 60\n5\n50 65 77 90 102";
    pub fn format_input(input: &str) -> (Vec<i32>, Vec<i32>) {
        let splited: Vec<&str> = input.split("\n").collect();
        let ranked = splited[1];
        let player = splited[3];
        let ranked: Vec<i32> = ranked
            .split_whitespace()
            .map(|str| str.parse::<i32>().unwrap())
            .collect();
        let player: Vec<i32> = player
            .split_whitespace()
            .map(|str| str.parse::<i32>().unwrap())
            .collect();
        (ranked, player)
    }
    pub fn test(input: &str, expected_result: Vec<i32>) {
        let (ranked, player) = format_input(input);
        let result = climbing_leaderboard(&ranked, &player);
        assert_eq!(result, expected_result);
    }
    #[test]
    fn first_case() {
        let expected_result = vec![6, 4, 2, 1];
        test(FIRST_CASE_INPUT, expected_result)
    }
    #[test]
    fn second_case() {
        let expected_result = vec![6, 5, 4, 2, 1];
        test(SECOND_CASE_INPUT, expected_result)
    }
    #[test]
    fn third_case() {
        let expected_result = THIRD_CASE_RESULT.to_vec();
        test(THIRD_CASE_INPUT, expected_result)
    }
}
