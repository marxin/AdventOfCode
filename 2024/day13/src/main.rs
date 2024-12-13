use std::ops::{Add, Mul, Sub};
#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
struct Point(i32, i32);

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

impl Mul<i32> for Point {
    type Output = Point;

    fn mul(self, rhs: i32) -> Self::Output {
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

    const LIMIT: i32 = 100;

    let mut total = 0;

    for game in content.split("\n\n") {
        let lines = game.lines().collect_vec();
        let button_a = parse_button(lines[0], '+');
        let button_b = parse_button(lines[1], '+');
        let prize = parse_button(lines[2], '=');

        let minimum = (0..=LIMIT)
            .cartesian_product((0..=LIMIT))
            .filter_map(|(x, y)| {
                let final_pos = button_a * x + button_b * y;
                if prize == final_pos {
                    Some(3 * x + y)
                } else {
                    None
                }
            })
            .min();
        if let Some(minimum) = minimum {
            total += minimum;
        }
    }

    dbg!(total);
}
