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

#[allow(unused)]
struct Rect {
    top_left: Point,
    bottom_right: Point,
}

impl Rect {
    #[allow(dead_code)]
    fn contains(&self, p: &Point) -> bool {
        self.top_left.0 <= p.0
            && p.0 < self.bottom_right.0
            && self.top_left.1 <= p.1
            && p.1 < self.bottom_right.1
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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Robot {
    pos: Point,
    vel: Point,
}

const WIDTH: i64 = 101;
const HEIGHT: i64 = 103;

impl Robot {
    fn new(line: &str) -> Self {
        let parts = line.split_ascii_whitespace().collect_vec();
        let p = parts[0][2..].split(',').collect_vec();
        let v = parts[1][2..].split(',').collect_vec();

        Self {
            pos: Point(p[0].parse().unwrap(), p[1].parse().unwrap()),
            vel: Point(v[0].parse().unwrap(), v[1].parse().unwrap()),
        }
    }
}

fn print_robots(robots: &HashSet<Point>) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if robots.contains(&Point(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
    println!();
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let lines = content.lines().collect_vec();

    let mut robots = lines.iter().map(|line| Robot::new(line)).collect_vec();

    // let mut seen = HashSet::new();

    for i in 0..10000 {
        robots = robots
            .into_iter()
            .map(|r| {
                let mut x = (r.pos.0 + r.vel.0) % WIDTH;
                if x < 0 {
                    x += WIDTH;
                }
                let mut y = (r.pos.1 + r.vel.1) % HEIGHT;
                if y < 0 {
                    y += HEIGHT;
                }
                Robot {
                    pos: Point(x, y),
                    vel: r.vel,
                }
            })
            .collect_vec();

        let set: HashSet<Point> = robots.iter().map(|r| r.pos).collect();
        let x = i + 1;
        if (x - 1033) % WIDTH == 0 {
            println!("#{}", i + 1);
            print_robots(&set);
        }
    }

    const QUADRANTS: [Rect; 4] = [
        Rect {
            top_left: Point(0, 0),
            bottom_right: Point(WIDTH / 2, HEIGHT / 2),
        },
        Rect {
            top_left: Point(WIDTH - WIDTH / 2, 0),
            bottom_right: Point(WIDTH, HEIGHT / 2),
        },
        Rect {
            top_left: Point(0, HEIGHT - HEIGHT / 2),
            bottom_right: Point(WIDTH / 2, HEIGHT),
        },
        Rect {
            top_left: Point(WIDTH - WIDTH / 2, HEIGHT - HEIGHT / 2),
            bottom_right: Point(WIDTH, HEIGHT),
        },
    ];

    // dbg!(&robots);

    let counts = QUADRANTS
        .iter()
        .map(|q| robots.iter().filter(|r| q.contains(&r.pos)).count())
        .collect_vec();

    dbg!(&counts);
    dbg!(counts.iter().product::<usize>());
}
