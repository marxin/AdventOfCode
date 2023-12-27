#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;
use itertools::put_back;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
struct Point(i32, i32);

#[allow(dead_code)]
const MOVES: [Point; 4] = [Point(0, 1), Point(1, 0), Point(0, -1), Point(-1, 0)];

fn split_in_half(edges: &HashMap<String, Vec<String>>, cut: Vec<(String, String)>) -> usize {
    let start = edges.keys().next().unwrap().clone();
    let mut seen = HashSet::new();
    let mut worklist = VecDeque::from_iter(vec![start]);

    while let Some(item) = worklist.pop_front() {
        if !seen.contains(&item) {
            seen.insert(item.clone());
            for n in edges.get(&item).unwrap() {
                let edge = (item.clone(), n.clone());
                let edge2 = (n.clone(), item.clone());
                if !cut.contains(&edge) && !cut.contains(&edge2) {
                    if !seen.contains(n) {
                        worklist.push_back(n.clone());
                    }
                }
            }
        }
    }

    return seen.len();
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let lines = content.lines().collect_vec();
    let mut edges: HashMap<String, Vec<String>> = HashMap::new();
    let mut edge_list = Vec::new();

    for line in lines {
        let parts = line.split(':').collect_vec();
        let src = parts[0].to_string();
        for dest in parts[1].split_whitespace() {
            let dest = dest.to_string();
            edges.entry(src.clone()).and_modify(|e| e.push(dest.clone())).or_insert(vec![dest.clone()]);
            edges.entry(dest.clone()).and_modify(|e| e.push(src.clone())).or_insert(vec![src.clone()]);
            edge_list.push((src.clone(), dest.clone()));
        }
    }

    for (k, v) in edges.iter() {
        println!("{}", v.len());
    }

    todo!();

    for i in 0..edge_list.len() {
        for j in 0..edge_list.len() {
            for k in 0..edge_list.len() {
                let cut = vec![edge_list[i].clone(), edge_list[j].clone(), edge_list[k].clone()];
                let n = split_in_half(&edges, cut);
                println!("{i}/{j}/{k}");
                if n != 1481 {
                    println!("{n}");
                    println!("{n}");
                    panic!();
                }
            }
        }
    }

    println!("{edges:?}");
    println!("{edge_list:?}");
}
