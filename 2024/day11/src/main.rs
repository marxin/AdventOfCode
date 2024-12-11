use std::ops::{Add, Sub};
#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;

fn map_numbers(numbers: &[u64]) -> Vec<u64> {
    numbers
        .iter()
        .flat_map(|n| {
            let digits = n.to_string();
            let l = digits.len();
            match (n, l) {
                (0, _) => vec![1],
                (_, l) if l % 2 == 0 => {
                    vec![
                        digits[..l / 2].parse().unwrap(),
                        digits[l / 2..].parse().unwrap(),
                    ]
                }
                (n, _) => {
                    vec![2024 * n]
                }
                _ => todo!("unexpected number"),
            }
        })
        .collect_vec()
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let mut numbers = content
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect_vec();

    for _ in 0..25 {
        numbers = map_numbers(&numbers);
    }
    dbg!(numbers.len());
}
