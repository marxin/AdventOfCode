use std::ops::{Add, Mul, RangeInclusive, Sub};
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
    let (ranges, ids) = content.split_once("\n\n").unwrap();

    let ranges = ranges
        .lines()
        .map(|part: &str| -> RangeInclusive<i64> {
            let (s, e) = part.split_once('-').unwrap();
            s.parse().unwrap()..=e.parse().unwrap()
        })
        .collect_vec();
    let numbers = ids.lines().map(|l| l.parse::<i64>().unwrap()).collect_vec();

    let count = numbers
        .iter()
        .filter(|n| ranges.iter().any(|r| r.contains(n)))
        .count();
    println!("{count}");
}
