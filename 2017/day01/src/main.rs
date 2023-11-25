#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, fs};

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn main() {
    let mut line = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .to_string();
    line.push(line.chars().next().unwrap());

    let values: Vec<_> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let s: u32 = values
        .windows(2)
        .filter_map(|pair| {
            if pair[0] == pair[1] {
                Some(pair[1])
            } else {
                None
            }
        })
        .sum();

    println!("{s:?}");
}
