#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};
use itertools::Itertools;

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    for line in content.lines() {
        println!("{line}")
    }
}
