#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, fs};

use itertools::Itertools;

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn main() {
    // c dec -10 if a >= 1

    let mut registers = HashMap::new();
    let data = fs::read_to_string("input.txt").unwrap();
    let mut maximum = i32::MIN;

    for line in data.lines() {
        let (dst, insn, by, _, lhs, op, rhs) = line.split_whitespace().collect_tuple().unwrap();
        let lhs = *registers.entry(lhs).or_insert(0);
        let rhs: i32 = rhs.parse().unwrap();
        let by: i32 = by.parse().unwrap();

        let add = match insn {
            "inc" => by,
            "dec" => -by,
            _ => panic!()
        };

        let op = match op {
            "==" => lhs == rhs,
            "!=" => lhs != rhs,
            "<" => lhs < rhs,
            "<=" => lhs <= rhs,
            ">" => lhs > rhs,
            ">=" => lhs >= rhs,
            _ => panic!(),
        };

        if op {
            *registers.entry(dst).or_insert(0) += add;
        }

        let m = *registers.values().max().unwrap();
        maximum = maximum.max(m);
    }

    println!("{registers:?}");
    println!("max register={}", registers.values().max().unwrap());
    println!("max during={}", maximum);
}
