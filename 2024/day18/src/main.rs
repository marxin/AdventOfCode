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
    let lines = content.lines().collect_vec();

    let mut map = HashSet::new();

    const WIDTH: i64 = 71;
    const HEIGHT: i64 = WIDTH;

    #[allow(unused)]
    for line in lines.iter().take(1024) {
        let parts = line.split_once(",").unwrap();
        map.insert(Point(parts.0.parse().unwrap(), parts.1.parse().unwrap()));
    }
    assert_eq!(map.len(), 1024);

    let mut start = Point(0, 0);
    let end = Point(WIDTH - 1, HEIGHT - 1);

    let mut steps = HashMap::from([(start, 0)]);
    let mut worklist = VecDeque::from([start]);

    while let Some(step) = worklist.pop_front() {
        let s = steps[&step];
        for m in MOVES.iter() {
            let next = step + *m;
            if next.0 >= 0
                && next.0 < WIDTH
                && next.1 >= 0
                && next.1 < HEIGHT
                && !steps.contains_key(&next)
                && !map.contains(&next)
            {
                steps.insert(next, s + 1);
                worklist.push_back(next);
            }
        }
    }

    dbg!(steps[&end]);
}
