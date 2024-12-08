use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

use itertools::Itertools;

type AreaMap = Vec<Vec<u8>>;
type Point = (usize, usize);

fn main() {
    let input = std::io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().bytes().collect_vec())
        .collect_vec();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &AreaMap) -> i32 {
    solve(input, calculate_anti_pair_coords_exact)
}

fn part2(input: &AreaMap) -> i32 {
    solve(input, calculate_anti_pair_coords_all_line)
}

fn solve(input: &AreaMap, antinode_calc: fn(&AreaMap, &Point, &Point) -> Vec<Point>) -> i32 {
    let mut pairs: HashMap<u8, Vec<Point>> = HashMap::new();
    let mut anti_pairs: HashSet<(usize, usize)> = HashSet::new();
    for (y, row) in input.iter().enumerate() {
        for (x, cell) in row.iter().enumerate().filter(|(_, cell)| **cell != b'.') {
            if let Some(pairs) = pairs.get(cell) {
                for pair in pairs {
                    let anti_pair_coords = antinode_calc(input, pair, &(x, y));
                    for anti_pair_coords in anti_pair_coords {
                        anti_pairs.insert(anti_pair_coords);
                    }
                }
            }
            pairs.entry(*cell).or_default().push((x, y));
        }
    }
    anti_pairs.len() as i32
}

fn calculate_anti_pair_coords_exact(
    input: &AreaMap,
    (x1, y1): &Point,
    (x2, y2): &Point,
) -> Vec<Point> {
    let dx = *x2 as i32 - *x1 as i32;
    let dy = *y2 as i32 - *y1 as i32;

    let x3 = *x2 as i32 + dx;
    let y3 = *y2 as i32 + dy;

    let x4 = *x1 as i32 - dx;
    let y4 = *y1 as i32 - dy;

    let mut result = vec![];
    if is_in_bounds(input, x3, y3) {
        result.push((x3 as usize, y3 as usize));
    }

    if is_in_bounds(input, x4, y4) {
        result.push((x4 as usize, y4 as usize));
    }

    result
}

fn calculate_anti_pair_coords_all_line(
    input: &AreaMap,
    (x1, y1): &Point,
    (x2, y2): &Point,
) -> Vec<Point> {
    let dx = *x2 as i32 - *x1 as i32;
    let dy = *y2 as i32 - *y1 as i32;

    let mut result = vec![];

    let mut x3 = *x2 as i32 + dx;
    let mut y3 = *y2 as i32 + dy;

    while is_in_bounds(input, x3, y3) {
        result.push((x3 as usize, y3 as usize));
        x3 += dx;
        y3 += dy;
    }

    let mut x4 = *x1 as i32 - dx;
    let mut y4 = *y1 as i32 - dy;

    while is_in_bounds(input, x4, y4) {
        result.push((x4 as usize, y4 as usize));
        x4 -= dx;
        y4 -= dy;
    }

    result.push((*x1, *y1));
    result.push((*x2, *y2));

    result
}

fn is_in_bounds(input: &AreaMap, x: i32, y: i32) -> bool {
    x >= 0 && x < input[0].len() as i32 && y >= 0 && y < input.len() as i32
}
