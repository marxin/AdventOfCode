#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
struct Point(i32, i32);

#[allow(dead_code)]
const MOVES: [Point; 4] = [Point(0, 1), Point(1, 0), Point(0, -1), Point(-1, 0)];

fn step(map: &HashSet<(i32, i32)>, positions: HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let mut next = HashSet::new();

    for p in positions.iter() {
        for m in MOVES {
            let next_pos = (p.0 + m.0, p.1 + m.1);
            if map.contains(&next_pos) {
                next.insert(next_pos);
            }
        }
    }

    next
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let lines = content.lines().collect_vec();

    let width = lines.first().unwrap().len() as i32;
    let height = lines.len() as i32;

    let mut start = None;
    let mut map = HashSet::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = (x as i32, y as i32);
            match c {
                '#' => {},
                '.' => {
                    map.insert(pos);
                },
                'S' => {
                    start = Some(pos);
                    map.insert(pos);
                },
                _ => todo!()
            }
        }
    }

    let mut positions: HashSet<(i32, i32)> = HashSet::from_iter(vec![start.unwrap()]);

    for _ in 0..64 {
        positions = step(&map, positions);
    }

    println!("{}", positions.len());
}
