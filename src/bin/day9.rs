use std::{collections::VecDeque, io::BufRead as _};

use itertools::Itertools;

fn main() {
    let input = std::io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .map(|l| l.to_digit(10).unwrap() as i64)
        .collect_vec();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &[i64]) -> i64 {
    let mut disk: Vec<(i64, i64)> = Vec::new();

    let mut id = 0;
    for (i, v) in input.iter().enumerate() {
        if i % 2 == 0 {
            disk.push((id, *v));
            id += 1;
        } else {
            disk.push((-1, *v));
        }
    }

    let mut disk_files_only = disk
        .iter()
        .filter(|(id, _)| *id != -1)
        .copied()
        .collect::<VecDeque<_>>();

    let mut new_disk = Vec::new();
    for fragment in disk.iter() {
        if fragment.0 == -1 {
            let mut to_fill = fragment.1;
            while to_fill > 0 {
                if let Some((id, val)) = disk_files_only.pop_back() {
                    if val >= to_fill {
                        new_disk.push((id, to_fill));
                        disk_files_only.push_back((id, val - to_fill));
                        break;
                    } else {
                        new_disk.push((id, val));
                        to_fill -= val;
                    }
                } else {
                    break;
                }
            }
        } else if let Some((id, val)) = disk_files_only.pop_front() {
            new_disk.push((id, val));
        } else {
            break;
        }
    }

    let mut checksum = 0;
    let mut i = 0;
    for (id, val) in new_disk.iter() {
        for _ in 0..*val {
            checksum += id * i;
            i += 1;
        }
    }
    checksum
}

fn part2(input: &[i64]) -> i64 {
    let mut disk: Vec<(i64, i64)> = Vec::new();

    let mut id = 0;
    for (i, v) in input.iter().enumerate() {
        if i % 2 == 0 {
            disk.push((id, *v));
            id += 1;
        } else {
            disk.push((-1, *v));
        }
    }

    let total_space: usize = disk.iter().fold(0, |acc, (_, v)| acc + *v as usize);
    let mut space: Vec<i64> = Vec::with_capacity(total_space);

    let mut disk_with_index = Vec::new();
    for (id, val) in disk.iter() {
        disk_with_index.push((space.len(), *id, *val));
        space.extend(std::iter::repeat(*id).take(*val as usize));
    }

    let disk_files_only = disk_with_index
        .iter()
        .filter(|(_, id, _)| *id != -1)
        .copied()
        .sorted_by_key(|(_, id, _)| *id)
        .rev()
        .collect_vec();

    for (index, id, len) in disk_files_only {
        fill(&mut space, index, id, len);
    }

    space
        .iter()
        .enumerate()
        .filter(|(_, id)| **id != -1)
        .fold(0, |acc, (i, id)| acc + id * i as i64)
}

fn fill(space: &mut [i64], index: usize, id: i64, len: i64) {
    if let Some(start_index) = find_contiguous_space(space, len) {
        if start_index > index {
            return;
        }
        for i in 0..len {
            space[start_index + i as usize] = id;
            space[index + i as usize] = -1;
        }
    }
}

fn find_contiguous_space(space: &[i64], len: i64) -> Option<usize> {
    let mut count = 0;
    for (i, &v) in space.iter().enumerate() {
        if v == -1 {
            count += 1;
            if count == len {
                return Some(i - len as usize + 1);
            }
        } else {
            count = 0;
        }
    }
    None
}
