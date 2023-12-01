#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn first_digit(line: &str) -> usize {
    for i in 0..line.len() {
        let c = line.chars().nth(i).unwrap();
        if c.is_digit(10) {
            return c.to_string().parse().unwrap();
        }
        for (n, d) in DIGITS.iter().enumerate() {
            if line[i..].starts_with(d) {
                return n + 1;
            }
        }
    }
    panic!()
}

fn last_digit(line: &str) -> usize {
    for i in (0..line.len()).rev() {
        let c = line.chars().nth(i).unwrap();
        if c.is_digit(10) {
            return c.to_string().parse().unwrap();
        }
        for (n, d) in DIGITS.iter().enumerate() {
            if line[..=i].ends_with(d) {
                return n + 1;
            }
        }
    }
    panic!()
}

fn main() {
    let mut sum = 0;
    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let number = 10 * first_digit(line) + last_digit(line);
        sum += number;
    }

    println!("{sum}");
}
