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

#[derive(Debug)]
enum Op {
    And,
    Or,
    Xor,
}

#[derive(Debug)]
struct Gate {
    op: Op,
    inputs: [String; 2],
    output: String,
}

fn value(known: &HashMap<String, bool>, prefix: &str) -> u64 {
    let mut result = 0u64;

    for (bit, value) in known.iter() {
        if let Some(suffix) = bit.strip_prefix(prefix) {
            let index = suffix.parse::<usize>().unwrap();
            if *value {
                result |= 1 << index;
            }
        }
    }

    result
}

fn compute(mut gates: Vec<Gate>, mut known: HashMap<String, bool>) -> u64 {
    while !gates.is_empty() {
        gates = gates
            .into_iter()
            .filter_map(|g| {
                let i1 = known.get(&g.inputs[0]);
                let i2 = known.get(&g.inputs[1]);
                if let (Some(i1), Some(i2)) = (i1, i2) {
                    let value = match g.op {
                        Op::And => i1 & i2,
                        Op::Or => i1 | i2,
                        Op::Xor => i1 ^ i2,
                    };
                    known.insert(g.output, value);
                    None
                } else {
                    Some(g)
                }
            })
            .collect_vec();
    }
    value(&known, "z")
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let (input, gates) = content.split_once("\n\n").unwrap();
    let mut known: HashMap<_, _> = input
        .lines()
        .map(|x| {
            let (name, value) = x.split_once(": ").unwrap();
            (name.to_string(), value.parse::<u64>().unwrap() == 1)
        })
        .collect();

    let mut gates = gates
        .lines()
        .map(|line| {
            let parts = line.split_whitespace().collect_vec();
            Gate {
                op: match parts[1] {
                    "AND" => Op::And,
                    "OR" => Op::Or,
                    "XOR" => Op::Xor,
                    _ => todo!(),
                },
                inputs: [parts[0].to_string(), parts[2].to_string()],
                output: parts[4].to_string(),
            }
        })
        .collect_vec();
    dbg!(gates.len());

    println!("digraph G{{ {{");
    for g in gates.iter() {
        let mut nodes = g.inputs.iter().collect_vec();
        nodes.push(&g.output);
        for node in nodes {
            let color = match node.chars().next().unwrap() {
                'x' => "[fillcolor=yellowgreen shape=box style=filled]",
                'y' => "[fillcolor=gold shape=circle style=filled]",
                'z' => "[fillcolor=deepskyblue shape=oval style=filled]",
                _ => "",
            };
            println!("{} {color}", node);
        }
    }
    println!("}}");

    for g in gates.iter() {
        let color = match g.op {
            Op::And => "[color=red]",
            Op::Or => "[color=yellowgreen]",
            Op::Xor => "[color=deepskyblue]",
        };
        println!("{} -> {} {}", g.inputs[0], g.output, color);
        println!("{} -> {} {}", g.inputs[1], g.output, color);
    }
    println!("}}");

    dbg!(value(&known, "y") + value(&known, "x"));
    dbg!(compute(gates, known));

    dbg!(
        ["vcg", "z24", "rvc", "rrs", "rkf", "z9", "jgb", "z20"]
            .iter()
            .sorted()
            .join(",")
    );
}
