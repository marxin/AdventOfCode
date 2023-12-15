#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn get_hash(s: &str) -> u8 {
    s.chars().fold(0, |acc, x| 17 * (acc + x as u8))
}

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let content = data.trim();
    println!("{}", content.split(',').map(|x| get_hash(x) as u64).sum::<u64>());
}
