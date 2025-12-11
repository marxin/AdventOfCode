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

fn find_best(
    joltage: Vec<i64>,
    buttons: &Vec<Vec<i64>>,
    pressed: usize,
    cache: &mut HashMap<Vec<i64>, Option<usize>>,
    winner: &mut usize,
) -> Option<usize> {
    if joltage.iter().all(|x| *x == 0) {
        return Some(pressed);
    } else if joltage.iter().any(|x| *x < 0) {
        return None;
    }

    let minimal = pressed + *joltage.iter().max().unwrap() as usize;
    if minimal >= *winner {
        return None;
    }

    if let Some(best) = cache.get(&joltage) {
        return *best;
    }

    let mut best = None;
    for button in buttons {
        let mut joltage2 = joltage.clone();
        for b in button {
            joltage2[*b as usize] -= 1;
        }

        if let Some(b) = find_best(joltage2, buttons, pressed + 1, cache, winner) {
            if let Some(b2) = best {
                if pressed == 0 && b2 < b {
                    *winner = b.min(b2);
                    println!("new winner: {}", *winner);
                }
                best = Some(b.min(b2));
            } else {
                if pressed == 0 {
                    *winner = b;
                    println!("new winner: {b}");
                }
                best = Some(b);
            }
        }
    }

    cache.insert(joltage, best);
    if cache.len() % 1_000_000 == 0 {
        dbg!(cache.len());
    }
    best
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let lines = content.lines().collect_vec();

    for line in lines {
        let parts = line.split_whitespace().collect_vec();

        let buttons = parts
            .iter()
            .skip(1)
            .rev()
            .skip(1)
            .rev()
            .map(|part| {
                part.strip_prefix("(")
                    .unwrap()
                    .strip_suffix(")")
                    .unwrap()
                    .split(",")
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect_vec()
            })
            .sorted_by_key(|x| x.len())
            .rev()
            .collect_vec();

        let joltage = parts
            .last()
            .unwrap()
            .strip_prefix("{")
            .unwrap()
            .strip_suffix("}")
            .unwrap()
            .split(",")
            .map(|n| n.parse::<i64>().unwrap())
            .collect_vec();

        let mut cache = HashMap::new();
        let mut best = usize::MAX;
        dbg!(find_best(joltage, &buttons, 0, &mut cache, &mut best));
    }
}
