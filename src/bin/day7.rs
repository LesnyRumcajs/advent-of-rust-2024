use std::io::BufRead as _;

use itertools::Itertools;
use rayon::prelude::*;

type Input = Vec<(i64, Vec<i64>)>;

fn main() {
    let input: Input = std::io::stdin()
        .lock()
        .lines()
        .map(|l| {
            let l = l.unwrap();
            let (value, nums) = l.split_once(':').unwrap();
            (
                value.parse().unwrap(),
                nums.split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect_vec(),
            )
        })
        .collect_vec();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &Input) -> i64 {
    solve(input, &['+', '*'])
}

fn part2(input: &Input) -> i64 {
    solve(input, &['+', '*', '|'])
}

fn solve(input: &Input, ops: &[char]) -> i64 {
    let biggest_count = input.par_iter().map(|(_, nums)| nums.len()).max().unwrap();
    let all_ops = itertools::repeat_n(ops, biggest_count - 1)
        .multi_cartesian_product()
        .collect_vec();

    input
        .par_iter()
        .filter(|(value, nums)| {
            all_ops.iter().any(|ops| {
                nums.iter()
                    .skip(1)
                    .zip(ops)
                    .fold(nums[0], |acc, (num, &op)| match op {
                        '+' => acc + num,
                        '*' => acc * num,
                        '|' => format!("{acc}{num}").parse().unwrap(),
                        _ => unreachable!(),
                    })
                    == *value
            })
        })
        .map(|(value, _)| value)
        .sum()
}
