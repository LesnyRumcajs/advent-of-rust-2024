use std::{collections::HashSet, io::BufRead};

use itertools::Itertools;

type Area = Vec<Vec<i8>>;
const TRAILHEAD: i8 = 0;
const PEAK: i8 = 9;

fn main() {
    let input = std::io::stdin()
        .lock()
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i8)
                .collect_vec()
        })
        .collect_vec();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &Area) -> i32 {
    let mut all_paths = 0;
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if input[i][j] == TRAILHEAD {
                let mut peaks = HashSet::new();
                find_path(input, &mut peaks, i, j, TRAILHEAD);
                all_paths += peaks.len() as i32;
            }
        }
    }
    all_paths
}

fn part2(input: &Area) -> i32 {
    let mut all_paths = 0;
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if input[i][j] == TRAILHEAD {
                all_paths += find_path_rating(input, i, j, TRAILHEAD);
            }
        }
    }
    all_paths
}

fn find_path(
    area: &Area,
    peaks: &mut std::collections::HashSet<(usize, usize)>,
    i: usize,
    j: usize,
    target: i8,
) {
    if area[i][j] == PEAK {
        peaks.insert((i, j));
        return;
    }

    for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
        let new_i = i as i32 + di;
        let new_j = j as i32 + dj;
        if new_i >= 0
            && new_i < area.len() as i32
            && new_j >= 0
            && new_j < area[0].len() as i32
            && area[new_i as usize][new_j as usize] == target + 1
        {
            find_path(area, peaks, new_i as usize, new_j as usize, target + 1);
        }
    }
}

fn find_path_rating(area: &Area, i: usize, j: usize, target: i8) -> i32 {
    if area[i][j] == PEAK {
        return 1;
    }

    let mut paths = 0;
    for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
        let new_i = i as i32 + di;
        let new_j = j as i32 + dj;
        if new_i >= 0
            && new_i < area.len() as i32
            && new_j >= 0
            && new_j < area[0].len() as i32
            && area[new_i as usize][new_j as usize] == target + 1
        {
            paths += find_path_rating(area, new_i as usize, new_j as usize, target + 1);
        }
    }
    paths
}
