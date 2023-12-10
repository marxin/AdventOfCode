#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn main() {
    let mut total = 0;
    let content = fs::read_to_string("input.txt").unwrap();
    for line in content.lines() {
        let numbers = line.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect_vec();

        let mut lines = vec![numbers];
        loop {
            let last = lines.last().unwrap();
            if last.iter().all(|x| x == &0) {
                break;
            }

            lines.push(last.iter().tuple_windows().map(|(x, y)| y - x).collect_vec());
        }

        let next = lines.iter().map(|value| value.last().unwrap()).sum::<i64>();
        total += next;

        //println!("{lines:?}");
        //println!();
    }

    println!("{total}");
}
