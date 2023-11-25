use std::process;
#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, fs};

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];

const PUZZLE: u32 = 312051;

fn get_spiral_position(n: u32) -> u32 {
    let mut width = 1;
    let mut pixels = 1;
    let mut d = 0i32;

    while pixels < n {
        d += 1;
        width += 2;
        pixels += 2 * width + 2 * (width - 2);
        // println!("{d} {width}, {pixels}");
    }

    let mut todo = pixels - n;
    let mut pos = (d, d);

    loop {
        for m in MOVES {
            for _ in 0..width - 1 {
                if todo == 0 {
                    return (pos.0.abs() + pos.1.abs()) as u32;
                } else {
                    pos = (pos.0 + m.0, pos.1 + m.1);
                    todo -= 1;
                }
            }
        }
    }
}

fn neigh_sum(diagonal: &HashMap<(i32, i32), u32>, pos: (i32, i32)) -> u32 {
    let mut s = 0;
    for x in -1..=1 {
        for y in -1..=1 {
            let pos2 = (pos.0 + x, pos.1 + y);
            if let Some(v) = diagonal.get(&pos2) {
                s += v;
            }
        }
    }

    s
}

fn main() {
    println!("{}", get_spiral_position(PUZZLE));

    let mut diagonal = HashMap::new();
    diagonal.insert((0, 0), 1);

    let mut pos = (1, 0);
    let mut width = 3;
    println!("1");

    loop {
        for (i, m) in MOVES.iter().enumerate() {
            let d = match i {
                0 => width - 2,
                1 | 2 => width - 1,
                3 => width,
                _ => panic!(),
            };
            for _ in 0..d {
                let v = neigh_sum(&diagonal, pos);
                if v > PUZZLE {
                    println!("{v}");
                    process::exit(0);
                } else {
                    println!("{pos:?} {v}");
                    diagonal.insert(pos, v);
                    pos = (pos.0 + m.0, pos.1 + m.1);
                }
            }
        }
        // move to another circle
        width += 2;
    }
}
