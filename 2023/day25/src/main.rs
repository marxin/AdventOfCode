#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;

use rand::prelude::*;

fn reduce_graph(edges: &Vec<(usize, usize)>, vertices: usize, attempt: usize) -> bool {
    let mut rng = rand::thread_rng();
    let mut collapsed = vec![1; vertices];
    let mut edges = edges.clone();

    for _ in 0..vertices - 2 {
        let (node, removed) = edges.choose(&mut rng).unwrap();
        collapsed[*node] += collapsed[*removed];

        edges = edges
            .iter()
            .filter_map(|(src, dest)| {
                let src = if src == removed { *node } else { *src };
                let dest = if dest == removed { *node } else { *dest };
                if src == dest {
                    None
                } else {
                    Some((src, dest))
                }
            })
            .collect_vec();
    }

    println!("Attempt #{attempt} with edges: {}", edges.len());
    if edges.len() == 3 {
        let a = collapsed[edges[0].0];
        let b = collapsed[edges[0].1];
        println!("{a} x {b} = {}", a * b);
        true
    } else {
        false
    }
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let lines = content.lines().collect_vec();
    let mut vertices = HashMap::new();
    let mut edges = Vec::new();

    for line in lines {
        let parts = line.split(':').collect_vec();
        let src = parts[0].to_string();
        let count = vertices.len();
        let src = *vertices.entry(src).or_insert(count);
        for dest in parts[1].split_whitespace() {
            let dest = dest.to_string();
            let count = vertices.len();
            let dest = *vertices.entry(dest).or_insert(count);
            edges.push((src, dest));
        }
    }

    for i in 0.. {
        let done = reduce_graph(&edges, vertices.len(), i);
        if done {
            break;
        }
    }
}
