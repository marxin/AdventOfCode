#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
struct Point(i32, i32);

#[allow(dead_code)]
const MOVES: [Point; 4] = [Point(0, 1), Point(1, 0), Point(0, -1), Point(-1, 0)];

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let lines = content.lines().collect_vec();

    let _width = lines.first().unwrap().len() as i32;
    let _height = lines.len() as i32;

    let lines = lines
        .into_iter()
        .map(|l| {
            let mut parts = l.split_whitespace();
            (
                parts.next().unwrap().parse::<i64>().unwrap(),
                parts.next().unwrap().parse::<i64>().unwrap(),
            )
        })
        .collect_vec();

    let first = lines.iter().map(|x| x.0).sorted().collect_vec();
    let second = lines.iter().map(|x| x.1).sorted().collect_vec();

    let r: i64 = first.iter().zip(second).map(|(a, b)| (a - b).abs()).sum();
    dbg!(r);
}
