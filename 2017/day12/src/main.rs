#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn main() {
    let mut edges = HashMap::new();

    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let parts: Vec<_> = line.split(" <-> ").collect();
        let src = parts[0].parse::<i32>().unwrap();
        for dst in parts[1].split(", ") {
            let dst = dst.parse::<i32>().unwrap();
            edges.entry(src).and_modify(|e: &mut HashSet<i32>| { e.insert(dst); }).or_insert(HashSet::from_iter(vec![dst]));
        }
    }

    let mut queue = VecDeque::new();
    let mut seen: HashSet<i32> = HashSet::new();

    let mut group_id = 0;
    while seen.len() != edges.len() {
        for n in edges.keys() {
            if !seen.contains(n) {
                queue.push_back(n.clone());
                break;
            }
        }

        group_id += 1;
        while let Some(item) = queue.pop_back() {
            seen.insert(item);

            for child in &edges[&item] {
                if !seen.contains(&child) {
                    queue.push_back(child.clone());
                    seen.insert(child.clone());
                }
            }
        }
    }

    println!("{}", seen.len());
    println!("{}", group_id);
}
