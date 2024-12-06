use std::{collections::HashSet, io::BufRead as _};

use itertools::Itertools as _;

fn main() {
    let input = std::io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().into_bytes())
        .collect_vec();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[derive(Clone)]
struct AreaMap(Vec<Vec<u8>>);

#[derive(Clone)]
struct Guard {
    x: i32,
    y: i32,
    orientation: Orientation,
}

impl Guard {
    fn from_map(area: &AreaMap) -> Self {
        let (y, x) = area
            .0
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .find(|(_, cell)| **cell == b'^')
                    .map(|(x, _)| (y as i32, x as i32))
            })
            .unwrap();
        Self {
            x,
            y,
            orientation: Orientation::Up,
        }
    }
    fn move_forward(&mut self, area: &AreaMap) {
        if self.check_for_obstacle(area) {
            self.orientation.turn();
        }

        match self.orientation {
            Orientation::Up => self.y -= 1,
            Orientation::Down => self.y += 1,
            Orientation::Left => self.x -= 1,
            Orientation::Right => self.x += 1,
        }
    }

    fn is_out(&self, area: &AreaMap) -> bool {
        self.x < 0
            || self.y < 0
            || self.x >= area.0[0].len() as i32
            || self.y >= area.0.len() as i32
    }

    fn is_boundary(&self, area: &AreaMap) -> bool {
        self.x == 0
            || self.y == 0
            || self.x == area.0[0].len() as i32 - 1
            || self.y == area.0.len() as i32 - 1
    }

    fn check_for_obstacle(&self, area: &AreaMap) -> bool {
        !self.is_boundary(area)
            && match self.orientation {
                Orientation::Up => area.0[self.y as usize - 1][self.x as usize] == b'#',
                Orientation::Down => area.0[self.y as usize + 1][self.x as usize] == b'#',
                Orientation::Left => area.0[self.y as usize][self.x as usize - 1] == b'#',
                Orientation::Right => area.0[self.y as usize][self.x as usize + 1] == b'#',
            }
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

impl Orientation {
    fn turn(&mut self) {
        *self = match self {
            Orientation::Up => Orientation::Right,
            Orientation::Right => Orientation::Down,
            Orientation::Down => Orientation::Left,
            Orientation::Left => Orientation::Up,
        }
    }
}

fn part1(input: &[Vec<u8>]) -> i32 {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let area = AreaMap(input.to_vec());
    let mut guard = Guard::from_map(&area);

    while !guard.is_out(&area) {
        visited.insert((guard.x, guard.y));
        guard.move_forward(&area);
    }

    visited.len() as i32
}
fn part2(input: &[Vec<u8>]) -> i32 {
    let mut initial_visited: HashSet<(i32, i32, Orientation)> = HashSet::new();
    let mut area = AreaMap(input.to_vec());
    let initial_guard = Guard::from_map(&area);
    let mut guard = initial_guard.clone();

    while !guard.is_out(&area) {
        initial_visited.insert((guard.x, guard.y, guard.orientation));
        guard.move_forward(&area);
    }

    let mut combinations: HashSet<(i32, i32)> = HashSet::new();
    for (x, y, _) in initial_visited {
        let mut visited: HashSet<(i32, i32, Orientation)> = HashSet::new();
        visited.insert((x, y, Orientation::Up));
        let mut guard = initial_guard.clone();
        area.0[y as usize][x as usize] = b'#';

        while !guard.is_out(&area) {
            visited.insert((guard.x, guard.y, guard.orientation));
            if guard.check_for_obstacle(&area) {
                guard.orientation.turn();
                visited.insert((guard.x, guard.y, guard.orientation));
            }
            guard.move_forward(&area);
            if visited.contains(&(guard.x, guard.y, guard.orientation)) {
                combinations.insert((x, y));
                break;
            }
        }
        area.0[y as usize][x as usize] = b'.';
    }

    combinations.len() as i32
}
