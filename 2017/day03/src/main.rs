#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, fs};

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

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

fn main() {
    println!("{}", get_spiral_position(PUZZLE));
}
