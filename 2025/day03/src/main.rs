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

fn find_best(values: &[u64], total: u64, remaining: usize) -> u64 {
    if remaining == 0 {
        return total;
    }

    for digit in (0..=9).rev() {
        let positions = values
            .iter()
            .positions(|&d| d == digit)
            .filter(|&pos| values.len() - pos >= remaining)
            .collect_vec();
        if !positions.is_empty() {
            return positions
                .iter()
                .map(|&p| find_best(&values[p + 1..], 10 * total + values[p], remaining - 1))
                .max()
                .unwrap();
        }
    }

    unreachable!("must find a digit");
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let lines = content.lines().collect_vec();
    let mut total = 0;

    for line in lines {
        let digits = line
            .chars()
            .map(|d| d.to_string().parse::<u64>().unwrap())
            .collect_vec();

        total += find_best(&digits, 0, 12);
    }

    println!("{total}");
}
