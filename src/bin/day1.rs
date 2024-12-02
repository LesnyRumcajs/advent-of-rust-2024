use std::{collections::HashMap, io::BufRead as _};

use itertools::Itertools;

fn main() {
    let input = std::io::stdin()
        .lock()
        .lines()
        .map(|l| {
            l.unwrap()
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect_tuple::<(u32, u32)>()
                .unwrap()
        })
        .collect_vec();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &[(u32, u32)]) -> u32 {
    let column1_sorted = input.iter().map(|(a, _)| *a).sorted().collect_vec();
    let column2_sorted = input.iter().map(|(_, b)| *b).sorted().collect_vec();

    column1_sorted
        .iter()
        .zip(column2_sorted.iter())
        .map(|(&a, &b)| a.abs_diff(b))
        .sum()
}

fn part2(input: &[(u32, u32)]) -> u32 {
    let column2_freq_map = input
        .iter()
        .map(|(_, b)| *b)
        .fold(HashMap::new(), |mut map, b| {
            *map.entry(b).or_insert(0) += 1;
            map
        });

    input
        .iter()
        .map(|(a, _)| a * column2_freq_map.get(a).unwrap_or(&0))
        .sum()
}
