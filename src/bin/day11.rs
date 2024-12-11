use std::{collections::HashMap, io::BufRead};

use itertools::Itertools;

fn main() {
    let input = std::io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect_vec();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &[i64]) -> i64 {
    solve(input, 25)
}
fn part2(input: &[i64]) -> i64 {
    solve(input, 75)
}

type StoneMap = HashMap<i64, i64>;

fn solve(input: &[i64], n: usize) -> i64 {
    let mut all_stones: StoneMap = input.iter().copied().fold(HashMap::new(), |mut acc, x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    });

    for _ in 0..n {
        let mut new_stones: StoneMap = HashMap::new();
        for (stone, count) in all_stones.iter() {
            for new_stone in change_stone(*stone) {
                *new_stones.entry(new_stone).or_insert(0) += *count;
            }
        }
        all_stones = new_stones;
    }
    all_stones.values().sum()
}

fn change_stone(stone: i64) -> Vec<i64> {
    if stone == 0 {
        vec![1]
    } else if (stone.checked_ilog10().unwrap_or(0) + 1) % 2 == 0 {
        let stone = stone.to_string();
        let (stone1, stone2) = stone.split_at(stone.len() / 2);
        [stone1, stone2]
            .iter()
            .map(|x| x.parse::<i64>().unwrap())
            .collect_vec()
    } else {
        vec![stone * 2024]
    }
}
