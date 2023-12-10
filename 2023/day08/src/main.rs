#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;

use num::integer;

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn get_end_visits<'a>(
    mut pos: &'a str,
    map: &HashMap<&'a str, Vec<&'a str>>,
    insns: &Vec<char>,
) -> usize {
    let mut steps = 0;

    for insn in insns.iter().cycle() {
        pos = if *insn == 'L' {
            map[pos][0]
        } else {
            map[pos][1]
        };

        steps += 1;
        if pos.ends_with('Z') {
            return steps;
        }
    }

    panic!();
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let mut it = content.lines();
    let insns = it.next().unwrap().chars().collect_vec();
    it.next();
    let mut map = HashMap::new();

    while let Some(line) = it.next() {
        let parts = line.split(" = ").collect_vec();
        let src = parts[0];
        let dest = parts[1]
            .trim_matches(|c| c == '(' || c == ')')
            .split(", ")
            .collect_vec();
        map.insert(src, dest);
    }

    let positions = map
        .keys()
        .filter(|k| k.ends_with('A'))
        .cloned()
        .collect_vec();
    let mut lcm = 1;
    for position in positions {
        let steps = get_end_visits(position, &map, &insns);
        lcm = integer::lcm(lcm, steps);
        println!("{position}: {:?}", steps);
    }

    println!("{lcm}");
}
