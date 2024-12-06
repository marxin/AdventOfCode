#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
struct Point(i32, i32);

#[allow(dead_code)]
const MOVES: [Point; 4] = [Point(0, -1), Point(1, 0), Point(0, 1), Point(-1, 0)];

fn is_looping(mut pos: Point, map: &HashMap<Point, char>, wall: Point) -> bool {
    let mut direction = 0;

    let mut visited = HashSet::new();
    loop {
        let tuple = (pos, direction);
        if visited.contains(&tuple) {
            return true;
        } else {
            visited.insert(tuple);
        }

        let step = MOVES[direction];
        let next = Point(pos.0 + step.0, pos.1 + step.1);
        let value = map.get(&next);
        let Some(value) = value else {
            return false;
        };
        if *value == '#' || next == wall {
            direction = (direction + 1) % MOVES.len();
        } else {
            pos = next;
        }
    }
}

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

    let pos = pos.unwrap();
    let candidates = map
        .iter()
        .filter(|&(k, v)| *k != pos && *v == '.')
        .map(|x| x.0)
        .collect_vec();

    dbg!(
        candidates
            .iter()
            .filter(|c| { is_looping(pos, &map, ***c) })
            .count()
    );
}
