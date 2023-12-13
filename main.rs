#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let lines = content.lines().collect_vec();

    let _width = lines.first().unwrap().len() as i32;
    let _height = lines.len() as i32;

    for (y, line) in lines.iter().enumerate() {
        println!("{line}")
    }
}
