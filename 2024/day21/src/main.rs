#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};
use std::{
    iter,
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

fn stortest_path(
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
                stortest_path(known, next, end, steps + 1, path, paths);
                path.pop();
            }
        }
    }
}

fn find_all_shortest_paths(map: &str) -> HashMap<(char, char), Vec<Vec<char>>> {
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
            //if start_c == end_c {
            //return None;
            //}

            let start = positions[start_c];
            let end = positions[end_c];

            let mut paths = Vec::new();
            stortest_path(
                &positions.values().copied().collect::<HashSet<Point>>(),
                start,
                end,
                0,
                &mut Vec::new(),
                &mut paths,
            );
            let paths = paths
                .into_iter()
                .map(|mut path| {
                    path.insert(0, start);
                    path.push(end);
                    let path = path
                        .iter()
                        .tuple_windows()
                        .map(|(s, e)| *e - *s)
                        .collect_vec();
                    map_to_arrows(&path)
                })
                .collect_vec();

            Some(((*start_c, *end_c), paths))
        })
        .collect()
}

fn find_code(
    code: &str,
    paths_a: &HashMap<(char, char), Vec<Vec<char>>>,
    paths_b: &HashMap<(char, char), Vec<Vec<char>>>,
) -> usize {
    let mut total = 0;
    for (s, e) in iter::once('A').chain(code.chars()).tuple_windows() {
        total += paths_a[&(s, e)]
            .iter()
            .map(|path| steps(path, &paths_b, 2))
            .min()
            .unwrap();
    }
    total
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let lines = content.lines().collect_vec();

    let paths_a = find_all_shortest_paths(KEYBOARD1);
    let paths_b = find_all_shortest_paths(KEYBOARD2);

    let mut total = 0;
    for code in lines {
        let digits = String::from_iter(code.chars().filter(|x| x.is_digit(10)));
        let best = find_code(code, &paths_a, &paths_b);
        total += digits.parse::<usize>().unwrap() * best;
    }
    dbg!(total);
}

fn map_to_arrows(path: &[Point]) -> Vec<char> {
    path.iter()
        .map(|p| match p {
            Point(1, 0) => '>',
            Point(0, 1) => 'v',
            Point(-1, 0) => '<',
            Point(0, -1) => '^',
            Point(0, 0) => 'A',
            _ => panic!(),
        })
        .collect_vec()
}

fn steps(path: &[char], paths_b: &HashMap<(char, char), Vec<Vec<char>>>, level: i32) -> usize {
    //dbg!(String::from_iter(path));
    //dbg!(level);
    if level == 0 {
        return path.len();
    }

    let mut total = 0;
    for (s, e) in iter::once(&'A').chain(path.iter()).tuple_windows() {
        total += paths_b[&(*s, *e)]
            .iter()
            .map(|path| steps(path, &paths_b, level - 1))
            .min()
            .unwrap();
    }

    total
}
