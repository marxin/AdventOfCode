#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
struct Point(i32, i32);

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

fn can_solve(total: u64, target: u64, values: &[u64]) -> bool {
    if values.is_empty() {
        return total == target;
    }

    let (v, values) = values.split_first().unwrap();
    if can_solve(total + v, target, values) || can_solve(total * v, target, values) {
        true
    } else {
        let v = format!("{total}{v}");
        can_solve(v.parse().unwrap(), target, values)
    }
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let lines = content.lines().collect_vec();

    let mut total = 0;
    for line in lines {
        let (goal, values) = line.split_once(":").unwrap();
        let goal = goal.parse().unwrap();
        let values = values
            .split_whitespace()
            .map(|t| t.parse().unwrap())
            .collect_vec();
        if can_solve(0, goal, &values) {
            total += goal;
        }
    }

    dbg!(total);
}
