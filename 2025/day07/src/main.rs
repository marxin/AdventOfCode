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
fn flood_fill<T: Clone, F: Fn(&Point, &T, &Point, &T) -> bool>(
    map: &HashMap<Point, T>,
    directions: &[Point],
    predicate: F,
) -> Vec<HashSet<Point>> {
    let mut groups = Vec::new();
    let mut visited = HashSet::new();

    for (point, c) in map.iter() {
        if visited.contains(point) {
            continue;
        }
        visited.insert(*point);

        let mut group = HashSet::from([*point]);
        let mut queue = VecDeque::from([*point]);

        while let Some(p) = queue.pop_front() {
            for m in directions.iter() {
                let next = p + *m;
                if let Some(v) = map.get(&next)
                    && predicate(&p, c, &next, v)
                    && !visited.contains(&next)
                {
                    queue.push_back(next);
                    group.insert(next);
                    visited.insert(next);
                }
            }
        }

        groups.push(group);
    }

    groups
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let lines = content.lines().collect_vec();

    let mut start = None;
    let mut splitters = HashSet::new();
    let mut playground = HashSet::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = Point(x as _, y as _);
            match c {
                'S' => {
                    playground.insert(pos);
                    start = Some(pos);
                }
                '.' => {
                    playground.insert(pos);
                }
                '^' => {
                    splitters.insert(pos);
                }
                _ => unreachable!(),
            }
        }
    }

    let mut splitted = 0;
    let mut worklist = vec![start.unwrap()];
    while let Some(pos) = worklist.pop() {
        if !playground.remove(&pos) {
            continue;
        }
        let pos2 = Point(pos.0, pos.1 + 1);
        if splitters.contains(&pos2) {
            let p1 = Point(pos2.0 - 1, pos2.1 + 1);
            worklist.push(p1);
            let p2 = Point(pos2.0 + 1, pos2.1 + 1);
            worklist.push(p2);
            splitted += 1;
        } else if playground.contains(&pos2) {
            worklist.push(pos2);
        }
    }

    println!("{splitted}");
}
