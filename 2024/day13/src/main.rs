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

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let parse_button = |line: &str, split: char| {
        let parts = line.split(split).flat_map(|x| x.split(',')).collect_vec();
        Point(parts[1].parse().unwrap(), parts[3].parse().unwrap())
    };

    const ADD: i64 = 10000000000000;

    let mut total = 0;

    for game in content.split("\n\n") {
        let lines = game.lines().collect_vec();
        let a = parse_button(lines[0], '+');
        let b = parse_button(lines[1], '+');
        let c = parse_button(lines[2], '=') + Point(ADD, ADD);
        //dbg!(a, b, c);

        let coeff_b = (a.0 * c.1 - c.0 * a.1) / (a.0 * b.1 - b.0 * a.1);
        let coeff_a = (c.0 - coeff_b * b.0) / a.0;
        // dbg!(coeff_a, coeff_b);
        let dx = c.0 - (a.0 * coeff_a + coeff_b * b.0);
        let dy = c.1 - (a.1 * coeff_a + coeff_b * b.1);

        if dx == 0 && dy == 0 {
            total += 3 * coeff_a + coeff_b;
        }
    }

    dbg!(total);
}
