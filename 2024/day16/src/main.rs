use std::{
    cmp::Reverse,
    collections::BinaryHeap,
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

#[derive(PartialEq, Eq, Hash)]
struct Conf {
    pos: Point,
    orient: Point,
    steps: usize,
}

impl PartialOrd for Conf {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Conf {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        Reverse(self.steps).cmp(&Reverse(other.steps))
    }
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let lines = content.lines().collect_vec();

    let _width = lines.first().unwrap().len() as i64;
    let _height = lines.len() as i64;

    let mut maze = HashSet::new();
    let mut start = None;
    let mut end = None;

    #[allow(unused)]
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = Point(x as i64, y as i64);
            match c {
                'S' => {
                    start = Some(pos);
                    maze.insert(pos);
                }
                'E' => {
                    end = Some(pos);
                    maze.insert(pos);
                }
                '.' => {
                    maze.insert(pos);
                }
                '#' => {}
                _ => todo!("unexpected token"),
            }
        }
    }

    let start = start.unwrap();
    let end = end.unwrap();
    let current = Conf {
        pos: start,
        orient: Point(1, 0),
        steps: 0,
    };

    let mut best_known = HashMap::from([((current.pos, current.orient), 0usize)]);
    let mut queue = BinaryHeap::from([current]);

    const ROTATE_PRICE: usize = 1000;
    while let Some(conf) = queue.pop() {
        let next = conf.pos + conf.orient;
        let mut candidates = Vec::new();

        if maze.contains(&next) {
            candidates.push(Conf {
                pos: next,
                orient: conf.orient,
                steps: conf.steps + 1,
            });
        }
        let orient_index = MOVES.iter().position(|&x| x == conf.orient).unwrap();
        candidates.push(Conf {
            pos: conf.pos,
            orient: MOVES[(orient_index + 1) % MOVES.len()],
            steps: conf.steps + ROTATE_PRICE,
        });
        let i = if orient_index == 0 {
            MOVES.len() - 1
        } else {
            orient_index - 1
        };
        candidates.push(Conf {
            pos: conf.pos,
            orient: MOVES[i],
            steps: conf.steps + ROTATE_PRICE,
        });

        for cand in candidates {
            let value = best_known
                .entry((cand.pos, cand.orient))
                .or_insert_with(|| usize::MAX);
            if cand.steps < *value {
                *value = cand.steps;
                queue.push(cand);
            }
        }
    }

    let ends = best_known
        .iter()
        .filter(|&((pos, _orient), v)| *pos == end)
        .collect_vec();
    dbg!(ends.iter().map(|x| x.1).min());
}
