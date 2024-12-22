use std::{
    cmp::Reverse,
    ops::{Add, Mul, Sub},
};
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
                if let Some(v) = map.get(&next) {
                    if predicate(&p, c, &next, v) && !visited.contains(&next) {
                        queue.push_back(next);
                        group.insert(next);
                        visited.insert(next);
                    }
                }
            }
        }

        groups.push(group);
    }

    groups
}

const MODULO: u64 = 16777216;

fn generate_diffs(mut n: u64) -> (Vec<u64>, Vec<i64>) {
    let mut values = vec![n % 10];
    let next = |mut n: u64| {
        n ^= n * 64;
        n %= MODULO;
        n ^= n / 32;
        n %= MODULO;
        n ^= n * 2048;
        n %= MODULO;
        n
    };

    for _ in 0..2000 {
        n = next(n);
        values.push(n % 10);
    }

    let diffs = values
        .iter()
        .tuple_windows()
        .map(|(x, y)| *y as i64 - *x as i64)
        .collect_vec();
    (values, diffs)
}

fn build_table(n: u64) -> HashMap<Vec<i64>, u64> {
    let (values, diffs) = generate_diffs(n);
    let mut map = HashMap::new();

    for i in 0..diffs.len() {
        let window = diffs.iter().skip(i).take(4).copied().collect_vec();
        if window.len() < 4 {
            break;
        }
        map.entry(window).or_insert(values[i + 4]);
    }

    map
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let numbers = content
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect_vec();

    let tables = numbers.iter().map(|n| build_table(*n)).collect_vec();
    let candidates = tables.iter().flat_map(|t| t.keys()).unique().collect_vec();
    dbg!(candidates.len());

    let best = candidates
        .iter()
        .map(|&c| {
            tables
                .iter()
                .map(|t| t.get(c).map_or(0, |v| *v))
                .sum::<u64>()
        })
        .max();

    dbg!(best);
}
