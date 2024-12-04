use std::io::BufRead as _;

use itertools::Itertools as _;

fn main() {
    let input = std::io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect_vec();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &[String]) -> i32 {
    let mut count = 0;
    for (y, row) in input.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            let x = x as i32;
            let y = y as i32;
            if ch == 'X' {
                // check horizontally forward
                count += check_for_mas(input, [(x + 1, y), (x + 2, y), (x + 3, y)]) as i32;

                // check horizontally backward
                count += check_for_mas(input, [(x - 1, y), (x - 2, y), (x - 3, y)]) as i32;

                // check vertically
                count += check_for_mas(input, [(x, y + 1), (x, y + 2), (x, y + 3)]) as i32;

                // check vertically down
                count += check_for_mas(input, [(x, y - 1), (x, y - 2), (x, y - 3)]) as i32;

                // check diagonally up-left
                count +=
                    check_for_mas(input, [(x - 1, y - 1), (x - 2, y - 2), (x - 3, y - 3)]) as i32;

                // check diagonally up-right
                count +=
                    check_for_mas(input, [(x + 1, y - 1), (x + 2, y - 2), (x + 3, y - 3)]) as i32;

                // check diagonally down-left
                count +=
                    check_for_mas(input, [(x - 1, y + 1), (x - 2, y + 2), (x - 3, y + 3)]) as i32;

                // check diagonally down-right
                count +=
                    check_for_mas(input, [(x + 1, y + 1), (x + 2, y + 2), (x + 3, y + 3)]) as i32;
            }
        }
    }
    count
}

fn part2(input: &[String]) -> i32 {
    let mut count = 0;
    for (y, row) in input.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            let x = x as i32;
            let y = y as i32;
            if ch == 'A' {
                let left_down_ok = check_input(input, x - 1, y - 1, 'M')
                    && check_input(input, x + 1, y + 1, 'S')
                    || check_input(input, x - 1, y - 1, 'S')
                        && check_input(input, x + 1, y + 1, 'M');

                let right_up_ok = check_input(input, x - 1, y + 1, 'M')
                    && check_input(input, x + 1, y - 1, 'S')
                    || check_input(input, x - 1, y + 1, 'S')
                        && check_input(input, x + 1, y - 1, 'M');

                count += (left_down_ok && right_up_ok) as i32;
            }
        }
    }
    count
}

fn check_for_mas(input: &[String], coords: [(i32, i32); 3]) -> bool {
    let mut xmas = "MAS".chars();
    for (x, y) in coords.iter() {
        if !check_input(input, *x, *y, xmas.next().unwrap()) {
            return false;
        }
    }
    true
}

fn check_input(input: &[String], x: i32, y: i32, target: char) -> bool {
    if x >= input[0].len() as i32 || y >= input.len() as i32 || x < 0 || y < 0 {
        return false;
    }

    input[y as usize].chars().nth(x as usize).unwrap() == target
}
