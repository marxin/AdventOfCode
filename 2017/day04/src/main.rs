#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, fs};
use itertools::{sorted, Itertools};

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn main() {
    let mut n = 0;
    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let words: Vec<_> = line.split_whitespace().map(|s| String::from_iter(s.chars().sorted())).collect();
        if words.len() == (HashSet::<String>::from_iter(words.iter().cloned()).len()) {
            n += 1;
        }
    }

    println!("{n}");
}
