use core::num;
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

fn find_smallers(light: u64, bits: usize, numbers: &Vec<u64>) -> usize {
    let check_mask = (1 << bits) - 1;

    for i in 0..numbers.len() {
        for comb in numbers.iter().combinations(i) {
            let mut product = 0;
            for mask in comb {
                product ^= *mask;
            }
            if product & check_mask == light {
                return i;
            }
        }
    }

    unreachable!()
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let lines = content.lines().collect_vec();

    let mut sum = 0;
    for line in lines {
        let parts = line.split_whitespace().collect_vec();

        let light = parts[0]
            .strip_prefix("[")
            .unwrap()
            .strip_suffix("]")
            .unwrap()
            .replace("#", "1")
            .replace(".", "0")
            .chars()
            .rev()
            .join("");
        let bits = light.len();
        let light = u64::from_str_radix(&light, 2).unwrap();

        let numbers = parts
            .iter()
            .skip(1)
            .rev()
            .skip(1)
            .rev()
            .map(|part| {
                let mut mask = 0u64;
                for bit in part
                    .strip_prefix("(")
                    .unwrap()
                    .strip_suffix(")")
                    .unwrap()
                    .split(",")
                    .map(|n| n.parse::<u64>().unwrap())
                {
                    mask |= 1 << bit;
                }
                mask
            })
            .collect_vec();

        sum += find_smallers(light, bits, &numbers);
    }

    println!("{sum}");
}
