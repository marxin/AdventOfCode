#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, fs};

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn main() {
    let line = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .to_string();
    let mut banks: Vec<_> = line
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let mut seen: HashSet<Vec<u32>> = HashSet::new();
    let mut steps = 0;
    let mut one_more_time = true;

    loop {
        if seen.contains(&banks) {
            if one_more_time {
                one_more_time = false;
                seen.clear();
                seen.insert(banks.clone());
                steps = 0;
            } else {
                break;
            }
        } else {
            seen.insert(banks.clone());
        }

        steps += 1;

        let max = banks.iter().max().unwrap();
        let mut pos = banks.iter().position(|&x| x == *max).unwrap();
        let mut n = banks[pos];
        banks[pos] = 0;
        pos = (pos + 1) % banks.len();

        while n > 0 {
            banks[pos] += 1;
            pos = (pos + 1) % banks.len();
            n -= 1;
        }
    }

    println!("done after {steps} steps");
}
