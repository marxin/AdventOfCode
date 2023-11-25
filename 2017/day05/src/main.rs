#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, fs};

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn main() {
    let mut jumps = Vec::new();
    for line in fs::read_to_string("input.txt").unwrap().lines() {
        jumps.push(line.parse::<i32>().unwrap());
    }

    println!("{jumps:?}");
    let mut pc: i32 = 0;
    let mut steps = 0;

    loop {
        // println!("pc={pc}");
        if pc < 0 || pc as usize >= jumps.len() {
            println!("exit {pc} steps={steps}");
            break;
        } else {
            let offset = jumps[pc as usize];
            jumps[pc as usize] += if offset >= 3 { -1 } else { 1 };
            pc += offset;
            steps += 1;
        }
    }
}
