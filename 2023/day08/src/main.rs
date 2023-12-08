#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let mut it = content.lines();
    let insns = it.next().unwrap().chars().collect_vec();
    it.next();
    let mut map = HashMap::new();

    while let Some(line) = it.next() {
        let parts = line.split(" = ").collect_vec();
        let src = parts[0];
        let dest = parts[1].trim_matches(|c| c == '(' || c == ')').split(", ").collect_vec();
        map.insert(src, dest);
    }

    let mut pos = "AAA";
    let mut steps = 0;

    for insn in insns.iter().cycle() {
        if pos == "ZZZ" {
            break;
        }

        pos = if *insn == 'L' { map[pos][0]  } else { map[pos][1] };
        steps += 1;
    }
    
    println!("{steps}");
}
