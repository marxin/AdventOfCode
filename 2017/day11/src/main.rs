#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(dead_code)]
const HEX_MOVES: [(&str, (i32, i32)); 6] = [("n", (0, -2)), ("ne", (1, -1)), ("se", (1, 1)), ("s", (0, 2)), ("sw", (-1, 1)), ("nw", (-1, -1))];

fn main() {
    let steps: Vec<_> = fs::read_to_string("input.txt").unwrap().trim().split(',').map(|s| s.to_string()).collect();
    let moves: HashMap<_, _> = HEX_MOVES.into_iter().collect();
    let mut pos = (0, 0);

    for step in steps {
        let step = moves[&step[..]];
        pos = (pos.0 + step.0, pos.1 + step.1);
    }

    println!("{pos:?}");
    let s = pos.0.abs() + pos.1.abs();
    println!("{s} {}", s / 2);
}
