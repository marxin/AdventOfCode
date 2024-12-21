use std::ops::{Add, Mul, Sub};
#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;
use itertools::Position;

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

const KEYBOARD1: &str = "
789
456
123
#0A
";

const KEYBOARD2: &str = "
#^A
<v>
";

fn best_path(
    known: &HashSet<Point>,
    pos: Point,
    end: Point,
    steps: usize,
    path: &mut Vec<Point>,
    paths: &mut Vec<Vec<Point>>,
) {
    if steps > 10 {
        return;
    }

    if let Some(first_path) = paths.first() {
        if steps > first_path.len() {
            return;
        }
    }

    if pos == end {
        if let Some(first_path) = paths.first() {
            if steps < first_path.len() {
                paths.clear();
                paths.push(path.clone());
            } else if steps == first_path.len() {
                paths.push(path.clone());
            }
        } else {
            paths.push(path.clone());
        }
    } else {
        for m in MOVES.iter() {
            let next = pos + *m;
            if known.contains(&next) {
                path.push(next);
                best_path(known, next, end, steps + 1, path, paths);
                path.pop();
            }
        }
    }
}

fn find_shortest_paths(map: &str) -> HashMap<(char, char), Vec<Vec<Point>>> {
    let mut positions = HashMap::new();
    for (y, line) in map.trim().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '#' {
                positions.insert(c, Point(x as i64, y as i64));
            }
        }
    }

    positions
        .keys()
        .cartesian_product(positions.keys())
        .filter_map(|(start_c, end_c)| {
            if start_c == end_c {
                return None;
            }

            let start = positions[start_c];
            let end = positions[end_c];

            let mut paths = Vec::new();
            best_path(
                &positions.values().copied().collect::<HashSet<Point>>(),
                start,
                end,
                0,
                &mut Vec::new(),
                &mut paths,
            );
            Some(((*start_c, *end_c), paths))
        })
        .collect()
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let lines = content.lines().collect_vec();

    dbg!(find_shortest_paths(KEYBOARD1));
}
