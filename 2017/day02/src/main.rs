#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, fs};

use itertools::{Itertools, MinMaxResult};

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn dividable_result(values: &Vec<u32>) -> u32 {
    values
        .iter()
        .cartesian_product(values)
        .filter_map(|(&x, &y)| {
            if x != y && x % y == 0 {
                Some(x / y)
            } else {
                None
            }
        })
        .next()
        .unwrap()
}

fn main() {
    let mut sum = 0;
    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let values = line
            .split_whitespace()
            .map(|f| f.parse::<u32>().unwrap())
            .collect_vec();
        sum += dividable_result(&values);
    }

    println!("{sum}");
}
