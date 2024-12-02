#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
struct Point(i32, i32);

#[allow(dead_code)]
const MOVES: [Point; 4] = [Point(0, 1), Point(1, 0), Point(0, -1), Point(-1, 0)];

fn is_safe(values: &[i64]) -> bool {
    let diffs = values
        .windows(2)
        .map(|pairs| pairs[0] - pairs[1])
        .collect_vec();

    diffs.iter().all(|x| x.abs() > 0 && x.abs() <= 3)
        && diffs.iter().map(|x| x.signum()).all_equal()
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let lines = content.lines().collect_vec();

    let mut count = 0;
    for line in lines {
        let values = line
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect_vec();

        if is_safe(&values) {
            count += 1;
        } else {
            for i in 0..values.len() {
                let mut values2 = values.clone();
                values2.remove(i);
                if is_safe(&values2) {
                    count += 1;
                    break;
                }
            }
        }
    }

    dbg!(count);
}
