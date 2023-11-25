#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, fs};

use itertools::{Itertools, MinMaxResult};

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn main() {
    let mut sum = 0;
    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let minmax = line.split_whitespace().map(|f| f.parse::<u32>().unwrap()).minmax();
        if let MinMaxResult::MinMax(min, max) = minmax {
            sum += max - min;
        } else {
            panic!();
        }
    }

    println!("{sum}");
}
