#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn main() {
    let mut sum = 0;
    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let parts: Vec<_> = line.split(':').nth(1).unwrap().split(" | ").collect();
        let source: HashSet<i32> = HashSet::from_iter(parts[0].split(' ').filter(|x| !x.is_empty()).map(|x| x.parse::<i32>().unwrap()));
        let target: HashSet<i32> = HashSet::from_iter(parts[1].split(' ').filter(|x| !x.is_empty()).map(|x| x.parse::<i32>().unwrap()));
        sum += 2i32.pow((source.intersection(&target).count() - 1) as u32)
    }

    println!("{sum}");
}
