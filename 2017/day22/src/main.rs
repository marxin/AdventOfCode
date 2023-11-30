#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

#[derive(Debug)]
enum State {
    Weakened,
    Infected,
    Flagged,
}

fn main() {
    let mut states = HashMap::new();
    let lines: Vec<_> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect();
    assert_eq!(lines.len(), lines[0].len());

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                states.insert((x as i32, y as i32), State::Infected);
            }
        }
    }

    let mut carrier = ((lines.len() / 2) as i32, (lines.len() / 2) as i32);
    let mut orientation = 0;
    let mut total = 0;

    for _ in 0..10000000 {
        if let Some(v) = states.get(&carrier) {
            match v {
                State::Weakened => {
                    states.insert(carrier.to_owned(), State::Infected);
                    total += 1;
                }
                State::Infected => {
                    states.insert(carrier.to_owned(), State::Flagged);
                    orientation = (orientation + 1) % MOVES.len();
                }
                State::Flagged => {
                    states.remove(&carrier);
                    orientation = (orientation + 2) % MOVES.len();
                }
                _ => panic!(),
            }
        } else {
            // Clean state
            states.insert(carrier.to_owned(), State::Weakened);
            orientation = (orientation - 1) % MOVES.len();
        }

        carrier = (
            carrier.0 + MOVES[orientation].0,
            carrier.1 + MOVES[orientation].1,
        );
    }

    println!("{total}");
}
