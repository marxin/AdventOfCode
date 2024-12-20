use std::ops::{Add, Mul, Sub};
#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
struct Point(i64, i64);

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul<i64> for Point {
    type Output = Point;

    fn mul(self, rhs: i64) -> Self::Output {
        Point(self.0 * rhs, self.1 * rhs)
    }
}

#[allow(dead_code)]
const MOVES: [Point; 4] = [Point(0, 1), Point(1, 0), Point(0, -1), Point(-1, 0)];

#[allow(dead_code)]
const MOVES_WITH_DIAGONAL: [Point; 8] = [
    Point(0, 1),
    Point(1, 0),
    Point(0, -1),
    Point(-1, 0),
    Point(1, 1),
    Point(1, -1),
    Point(-1, 1),
    Point(-1, -1),
];

#[allow(dead_code)]
fn fastest_path(map: &HashSet<Point>, start: &Point, end: &Point, hacks: (Point, Point)) -> usize {
    let mut todo = VecDeque::from([(*start, 0)]);
    let mut visited = HashMap::from([(*start, 0)]);

    while let Some((pos, steps)) = todo.pop_front() {
        for m in MOVES.iter() {
            let next = pos + *m;
            if !visited.contains_key(&next) {
                if map.contains(&next) {
                    visited.insert(next, steps + 1);
                    todo.push_back((next, steps + 1));
                } else if next == hacks.0 {
                    visited.insert(hacks.0, steps + 1);
                    todo.push_back((hacks.0, steps + 1));
                    visited.insert(hacks.1, steps + 2);
                    todo.push_back((hacks.1, steps + 2));
                }
            }
        }
    }

    visited[end]
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let lines = content.lines().collect_vec();

    let mut maze = HashSet::new();
    let mut walls = HashSet::new();
    let mut start = None;
    let mut end = None;

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = Point(x as i64, y as i64);
            match c {
                'S' => {
                    start = Some(pos);
                    maze.insert(pos);
                }
                'E' => {
                    end = Some(pos);
                    maze.insert(pos);
                }
                '.' => {
                    maze.insert(pos);
                }
                '#' => {
                    walls.insert(pos);
                }
                _ => todo!("unexpected token"),
            }
        }
    }

    let start = start.unwrap();
    let end = end.unwrap();

    const INV: i64 = -100;
    let default_path = fastest_path(&maze, &start, &end, (Point(INV, INV), Point(INV, INV)));
    dbg!(default_path);

    let candidates = walls
        .iter()
        .flat_map(|p| MOVES.iter().map(|m| (*p, *p + *m)))
        .filter(|x| maze.contains(&x.1) || walls.contains(&x.1))
        .collect_vec();

    let mut histogram: HashMap<usize, usize> = HashMap::new();

    for hack in candidates {
        let cheat = fastest_path(&maze, &start, &end, hack);
        if cheat < default_path {
            let diff = default_path - cheat;
            *histogram.entry(diff).or_default() += 1;
        }
    }

    for (k, v) in histogram.iter().sorted_by_key(|x| x.0) {
        println!("{k}:{v}");
    }
}
