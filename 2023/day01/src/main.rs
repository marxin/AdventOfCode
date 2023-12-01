#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn main() {
    let mut sum = 0;
    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let digits: Vec<_> = line.chars().filter(|c| c.is_digit(10)).map(|c| c.to_string().parse::<i32>().unwrap()).collect();
        let number = 10 * digits.first().unwrap() + digits.last().unwrap();
        sum += number;
    }

    println!("{sum}");
}
