use std::io::BufRead as _;

use itertools::Itertools as _;
use regex::Regex;

fn main() {
    let input = std::io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect_vec()
        .join("");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &str) -> i32 {
    calc(input)
}

fn part2(input: &str) -> i32 {
    let input = Regex::new(r"don't\(\).*?(do\(\)|$)")
        .unwrap()
        .replace_all(input, "");

    calc(&input)
}

fn calc(input: &str) -> i32 {
    Regex::new(r"mul\((\d+),(\d+)\)")
        .unwrap()
        .captures_iter(input)
        .fold(0, |acc, caps| {
            let a = caps[1].parse::<i32>().unwrap();
            let b = caps[2].parse::<i32>().unwrap();
            acc + a * b
        })
}
