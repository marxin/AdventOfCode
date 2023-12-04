#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn main() {
    let mut wins = HashMap::new();

    for (i, line) in fs::read_to_string("input.txt").unwrap().lines().enumerate() {
        let parts: Vec<_> = line.split(':').nth(1).unwrap().split(" | ").collect();
        let source: HashSet<i32> = HashSet::from_iter(
            parts[0]
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<i32>().unwrap()),
        );
        let target: HashSet<i32> = HashSet::from_iter(
            parts[1]
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<i32>().unwrap()),
        );
        wins.insert(i, source.intersection(&target).count());
    }

    let end = wins.len();
    let mut counts: HashMap<usize, usize> = HashMap::from_iter((0..end).map(|x| (x, 1usize)));
    for i in 0..end {
        let count = counts[&i];
        for next in 1..wins[&i] + 1 {
            *counts.get_mut(&(i + next)).unwrap() += count;
        }
    }

    println!("{:?}", counts.values().sum::<usize>());
}
