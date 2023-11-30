#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn main() {
    let mut infected = HashSet::new();
    let lines: Vec<_> = fs::read_to_string("input.txt").unwrap().lines().map(|l| l.to_string()).collect();
    assert_eq!(lines.len(), lines[0].len());

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                infected.insert((x as i32, y as i32));
            }
        }
    }

    let mut carrier = ((lines.len() / 2) as i32, (lines.len() / 2) as i32);
    let mut orientation = 0;
    let mut total = 0;

    for _ in 0..10000 {
        if infected.remove(&carrier) {
            orientation = (orientation + 1) % MOVES.len();
        } else {
            infected.insert(carrier.to_owned());
            orientation = (orientation - 1) % MOVES.len();
            total += 1;
        }

        carrier = (carrier.0 + MOVES[orientation].0, carrier.1 + MOVES[orientation].1);
    }

    println!("{infected:?}");
    println!("{total}");
}
