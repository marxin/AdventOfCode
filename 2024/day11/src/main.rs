#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;

fn map_numbers(numbers: &HashMap<u64, usize>) -> HashMap<u64, usize> {
    let mut output = HashMap::new();

    for (n, times) in numbers.iter() {
        let digits = n.to_string();
        let l = digits.len();
        let values = match (n, l) {
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
        };

        for v in values.into_iter() {
            *output.entry(v).or_default() += times;
        }
    }

    output
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let numbers = content
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect_vec();

    assert!(numbers.iter().all_unique());
    let mut numbers: HashMap<_, _> = numbers.into_iter().map(|n| (n, 1usize)).collect();

    for _ in 0..75 {
        numbers = map_numbers(&numbers);
    }

    dbg!(numbers.values().sum::<usize>());
}
