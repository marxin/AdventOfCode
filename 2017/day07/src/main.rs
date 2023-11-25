#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, fs};

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn main() {
    let mut names = HashSet::new();
    let mut seen = HashSet::new();

    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let parts: Vec<_> = line.split("-> ").collect();
        let name = parts[0].split_whitespace().next().unwrap().to_string();
        names.insert(name);
        if parts.len() == 2 {
            for x in parts.last().unwrap().split(", ") {
                seen.insert(x.to_string());
            }
        }
    }

    println!("{:?}", names.difference(&seen));
}
