#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
struct Point(i32, i32);

#[allow(dead_code)]
const MOVES: [Point; 4] = [Point(0, 1), Point(1, 0), Point(0, -1), Point(-1, 0)];

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let lines = content.lines().collect_vec();

    let mut map = HashMap::new();
    let mut pos = None;

    for (y, line) in lines.iter().enumerate() {
        for (x, mut c) in line.chars().enumerate() {
            let x = x as i32;
            let y = y as i32;
            if c == '^' {
                pos = Some(Point(x, y));
                c = '.';
            }

            map.insert(Point(x, y), c);
        }
    }

    let mut pos = pos.unwrap();
    let mut direction = 2;

    let mut visited = HashSet::new();
    loop {
        visited.insert(pos);
        let step = MOVES[direction];
        let next = Point(pos.0 + step.0, pos.1 + step.1);
        let value = map.get(&next);
        let Some(value) = value else {
            break;
        };
        if *value == '#' {
            direction = if direction == 0 {
                MOVES.len() - 1
            } else {
                direction - 1
            };
        } else {
            pos = next;
        }
    }

    dbg!(visited.len());
}
