#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let mut lineit = content.lines();
    let times = lineit.next().unwrap().split(':').last().unwrap().split_whitespace().collect_vec();
    let distances = lineit.next().unwrap().split(':').last().unwrap().split_whitespace().collect_vec();

    let mut result = 1;
    for (&time, distance) in times.iter().zip(distances) {
        let time = time.parse::<i32>().unwrap();
        let distance = distance.parse::<i32>().unwrap();

        let mut valid = 0;
        for wait in 1..time {
            if (time - wait) * wait > distance {
                valid += 1;
            }
        }

        println!("{valid}");
        result *= valid;
    }

    println!("{result}");
}
