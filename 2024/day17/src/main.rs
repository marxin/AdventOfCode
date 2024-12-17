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
    let parts = content.split_once("\n\n").unwrap();
    let mut regs = parts
        .0
        .lines()
        .map(|line| {
            line.split_whitespace()
                .last()
                .unwrap()
                .parse::<u64>()
                .unwrap()
        })
        .collect_vec();
    dbg!(&regs);
    let program = parts
        .1
        .trim()
        .split_once(": ")
        .unwrap()
        .1
        .split(",")
        .map(|x| x.parse::<u64>().unwrap())
        .collect_vec();

    let mut ip = 0;

    loop {
        let Some(&insn) = program.get(ip) else {
            break;
        };

        let operand = program[ip + 1];
        let op_value = match operand {
            v @ 0..=3 => v,
            id @ (4..=6) => regs[id as usize - 4],
            _ => todo!("unknown operand"),
        };

        match insn {
            0 => {
                regs[0] /= 2u64.pow(op_value as u32);
            }
            1 => {
                regs[1] ^= operand;
            }
            2 => {
                regs[1] = op_value % 8;
            }
            3 => {
                if regs[0] != 0 {
                    ip = operand as usize;
                    continue;
                }
            }
            4 => {
                regs[1] ^= regs[2];
            }
            5 => {
                print!("{},", op_value % 8);
            }
            6 => {
                regs[1] = regs[0] / 2u64.pow(op_value as u32);
            }
            7 => {
                regs[2] = regs[0] / 2u64.pow(op_value as u32);
            }
            _ => todo!("unknown insn"),
        }

        ip += 2;
    }
    println!();
    dbg!(regs);
}
