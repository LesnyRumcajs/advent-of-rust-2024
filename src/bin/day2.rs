use std::io::BufRead as _;

use itertools::Itertools;

fn main() {
    let input = std::io::stdin()
        .lock()
        .lines()
        .map(|l| {
            l.unwrap()
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect_vec()
        })
        .collect_vec();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &[Vec<u32>]) -> u32 {
    input
        .iter()
        .map(|row| {
            row.windows(2).all(|w| w[0] < w[1] && w[1] - w[0] <= 3)
                || row.windows(2).all(|w| w[0] > w[1] && w[0] - w[1] <= 3)
        })
        .filter(|&b| b)
        .count() as u32
}

fn part2(input: &[Vec<u32>]) -> u32 {
    input
        .iter()
        .map(|row| {
            for i in 0..row.len() {
                let mut row = row.clone();
                row.remove(i);

                if row.windows(2).all(|w| w[0] < w[1] && w[1] - w[0] <= 3)
                    || row.windows(2).all(|w| w[0] > w[1] && w[0] - w[1] <= 3)
                {
                    return true;
                }
            }

            false
        })
        .filter(|&b| b)
        .count() as u32
}
