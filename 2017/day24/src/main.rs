#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

fn find_best(bridges: Vec<(i32, i32)>, last: i32, total: i32, length: i32, best: &mut i32) {
    if length == 31 {
        if total > *best {
            *best = total;
        }
    }
    

    for (i, b) in bridges.iter().enumerate() {
        if b.0 == last || b.1 == last {
            let next;
            if b.0 == last {
                next = b.1;
            } else {
                next = b.0;
            }
            let mut bridges = bridges.clone();
            bridges.remove(i);
            find_best(bridges, next, total + b.0 + b.1, length + 1, best);
        }
    }
}

fn main() {
    let mut bridges = Vec::new();

    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let bridge: Vec<_> = line.split('/').map(|part| part.parse::<i32>().unwrap()).collect();
        bridges.push((bridge[0], bridge[1]));
    }

    println!("{bridges:?}");
    let mut best = 0;
    find_best(bridges, 0, 0, 0, & mut best);
    println!("BEST: {best}");
}
