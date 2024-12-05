use std::{collections::HashMap, io::BufRead};

use itertools::Itertools;

fn main() {
    let input = std::io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect_vec();
    let (rules, updates) = input.split_at(input.iter().position(|l| l.is_empty()).unwrap());

    println!("{}", part1(rules, updates));
    println!("{}", part2(rules, updates));
}

#[derive(Eq, PartialEq, Clone)]
struct Page<'a> {
    page: i32,
    rules: &'a HashMap<i32, Vec<i32>>,
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for Page<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.page == other.page {
            Some(std::cmp::Ordering::Equal)
        } else if self
            .rules
            .get(&self.page)
            .map_or(false, |v| v.contains(&other.page))
        {
            Some(std::cmp::Ordering::Less)
        } else {
            Some(std::cmp::Ordering::Greater)
        }
    }
}

impl Ord for Page<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn part1(rules: &[String], updates: &[String]) -> i32 {
    solve(rules, updates, UpdatesToApply::Correct)
}

fn part2(rules: &[String], updates: &[String]) -> i32 {
    solve(rules, updates, UpdatesToApply::Fixed)
}

#[derive(PartialEq)]
enum UpdatesToApply {
    Correct,
    Fixed,
}

fn solve(rules: &[String], updates: &[String], mode: UpdatesToApply) -> i32 {
    let mut rule_map: HashMap<i32, Vec<i32>> = HashMap::new();
    for rule in rules {
        let (key, value) = rule.split_once('|').unwrap();
        rule_map
            .entry(key.parse().unwrap())
            .or_default()
            .push(value.parse().unwrap());
    }

    let updates = updates
        .iter()
        .skip(1)
        .map(|l| {
            l.split(',')
                .map(|n| n.parse::<i32>().unwrap())
                .map(|page| Page {
                    page,
                    rules: &rule_map,
                })
                .collect_vec()
        })
        .collect_vec();

    updates.iter().rev().fold(0, |acc, update| {
        let sorted = update.iter().sorted().cloned().collect_vec();
        if sorted == *update {
            if mode == UpdatesToApply::Correct {
                acc + update[update.len() / 2].page
            } else {
                acc
            }
        } else if mode == UpdatesToApply::Fixed {
            acc + sorted[sorted.len() / 2].page
        } else {
            acc
        }
    })
}
