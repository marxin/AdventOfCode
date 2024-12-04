#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
struct Point(i32, i32);

#[allow(dead_code)]
const MOVES: [Point; 4] = [Point(0, 1), Point(1, 0), Point(0, -1), Point(-1, 0)];

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)|don't\(\)|do\(\)").unwrap();

    let mut enabled = true;
    let mut total = 0;

    for capture in regex.captures_iter(&content) {
        match capture.get(0).unwrap().as_str() {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                if enabled {
                    total += capture.get(1).unwrap().as_str().parse::<i64>().unwrap()
                        * capture.get(2).unwrap().as_str().parse::<i64>().unwrap();
                }
            }
        }
    }

    dbg!(total);
}
