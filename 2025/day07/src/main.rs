#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};
use std::{
    collections::vec_deque,
    ops::{Add, Mul, Sub},
};

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
    let width = lines[0].len();
    let height = lines.len();

    let mut splitters = HashMap::new();
    let mut playground = HashSet::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = Point(x as _, y as _);
            match c {
                'S' => {
                    playground.insert(pos);
                    splitters.insert(pos, 0);
                }
                '.' => {
                    playground.insert(pos);
                }
                '^' => {
                    splitters.insert(pos, 0);
                }
                _ => unreachable!(),
            }
        }
    }

    let mut total = 0;
    let mut worklist = (0..width)
        .map(|x| (Point(x as _, height as _), 1u64))
        .collect_vec();

    for y in (0..(height - 1)).rev() {
        // push particles up and then fill up newly the worklist
        while let Some((pos, times)) = worklist.pop() {
            let left = Point(pos.0 - 1, pos.1);
            splitters.entry(left).and_modify(|v| {
                *v += times;
            });

            let right = Point(pos.0 + 1, pos.1);
            splitters.entry(right).and_modify(|v| {
                *v += times;
            });

            let pos2 = Point(pos.0, pos.1 - 1);
            if pos2.1 < 0 {
                continue;
            }

            if splitters.contains_key(&pos2) {
                if pos2.1 == 0 {
                    // Start -> accumulate
                    total += times;
                }
                // end of the particle
            } else if playground.contains(&pos2) {
                worklist.push((pos2, times));
            }
        }

        for x in 0..width {
            let p = Point(x as _, y as _);
            if let Some(times) = splitters.get(&p) {
                worklist.push((p, *times));
            }
        }
    }
    println!("{total}");
}
