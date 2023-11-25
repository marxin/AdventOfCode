#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(dead_code)]
const HEX_MOVES: [(&str, (i32, i32)); 6] = [("n", (0, -2)), ("ne", (1, -1)), ("se", (1, 1)), ("s", (0, 2)), ("sw", (-1, 1)), ("nw", (-1, -1))];

fn get_distance(pos: (i32, i32)) -> i32 {
    (pos.0.abs() + pos.1.abs()) / 2
}

fn main() {
    let steps: Vec<_> = fs::read_to_string("input.txt").unwrap().trim().split(',').map(|s| s.to_string()).collect();
    let moves: HashMap<_, _> = HEX_MOVES.into_iter().collect();
    let mut pos = (0, 0);
    let mut max = 0;

    for step in steps {
        let step = moves[&step[..]];
        pos = (pos.0 + step.0, pos.1 + step.1);
        max = max.max(get_distance(pos));
    }

    println!("{pos:?}");
    println!("d={}", get_distance(pos));
    println!("max_d={}", max);
}
