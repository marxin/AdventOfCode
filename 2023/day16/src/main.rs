#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
struct Point(i32, i32);

#[allow(dead_code)]
const MOVES: [Point; 4] = [Point(0, 1), Point(1, 0), Point(0, -1), Point(-1, 0)];

fn reflect(c: char, orient: &Point) -> Vec<Point> {
    match c {
        '-' => match orient {
            &Point(1, 0) | &Point(-1, 0) => vec![orient.clone()],
            &Point(0, 1) | &Point(0, -1) => vec![Point(1, 0), Point(-1, 0)],
            _ => panic!(),
        },
        '|' => match orient {
            &Point(1, 0) | &Point(-1, 0) => vec![Point(0, 1), Point(0, -1)],
            &Point(0, 1) | &Point(0, -1) => vec![orient.clone()],
            _ => panic!(),
        },
        '/' => match orient {
            Point(1, 0) => vec![Point(0, -1)],
            Point(-1, 0) => vec![Point(0, 1)],
            Point(0, 1) => vec![Point(-1, 0)],
            Point(0, -1) => vec![Point(1, 0)],
            _ => panic!(),
        },
        '\\' => match orient {
            Point(1, 0) => vec![Point(0, 1)],
            Point(-1, 0) => vec![Point(0, -1)],
            Point(0, 1) => vec![Point(1, 0)],
            Point(0, -1) => vec![Point(-1, 0)],
            _ => panic!(),
        },
        '.' => vec![orient.clone()],
        _ => panic!(),
    }
}

fn compute_energy(map: &HashMap<Point, char>, start: Point, orient: Point) -> usize {
    let mut visited: HashSet<(Point, Point)> = HashSet::new();
    let mut queue = VecDeque::from([(start, orient)]);

    while let Some(state) = queue.pop_back() {
        if !visited.contains(&state) {
            visited.insert(state.clone());

            let (pos, orient) = state;
            for next_orient in reflect(map[&pos], &orient) {
                let next_pos = Point(pos.0 + next_orient.0, pos.1 + next_orient.1);
                if map.contains_key(&next_pos) && !visited.contains(&(next_pos, next_orient)) {
                    queue.push_back((next_pos, next_orient));
                }
            }
        }
    }

    visited.iter().map(|(p, _)| p.clone()).unique().count()
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let lines = content.lines().collect_vec();

    let width = lines.first().unwrap().len() as i32;
    let height = lines.len() as i32;
    let mut map = HashMap::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert(Point(x as i32, y as i32), c);
        }
    }

    let mut maximum = 0;
    for y in 0..height {
        let total = compute_energy(&map, Point(0, y), Point(1, 0));
        maximum = maximum.max(total);
        let total = compute_energy(&map, Point(width - 1, y), Point(-1, 0));
        maximum = maximum.max(total);
    }

    for x in 0..width {
        let total = compute_energy(&map, Point(x, 0), Point(0, 1));
        maximum = maximum.max(total);
        let total = compute_energy(&map, Point(x, height - 1), Point(0, -1));
        maximum = maximum.max(total);
    }

    println!("{maximum}");
}
