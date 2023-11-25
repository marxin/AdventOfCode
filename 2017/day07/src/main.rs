#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, fs};
use itertools::Itertools;

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn main() {
    let mut weights = HashMap::new();
    let mut nodes = HashMap::new();
    let mut known_totals = HashMap::new();

    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let parts: Vec<_> = line.split("-> ").collect();
        let name = parts[0].split_whitespace().next().unwrap().to_string();
        let weight = parts[0].split_ascii_whitespace().last().unwrap();
        let weight: usize = weight[1..weight.len() - 1].parse().unwrap();

        weights.insert(name.clone(), weight);
        if parts.len() == 2 {
            let children: HashSet<_> = parts.last().unwrap().split(", ").map(|x| x.to_string()).collect();
            nodes.insert(name, children);
        } else {
            known_totals.insert(name, weight);
        }
    }

    println!("{:?}", nodes);
    println!("{:?}", weights);
    println!("{:?}", known_totals);

    loop {
        for n in nodes.keys() {
            if !known_totals.contains_key(n) && nodes[n].iter().all(|x| known_totals.contains_key(x)) {
                let subtotals: Vec<_> = nodes[n].iter().map(|child| known_totals[child]).collect();
                let minmax = subtotals.iter().minmax();
                match minmax {
                    itertools::MinMaxResult::OneElement(&s) => {
                        known_totals.insert(n.clone(), s + weights[n]);
                    },
                    itertools::MinMaxResult::NoElements => panic!(),
                    itertools::MinMaxResult::MinMax(&min, &max) => {
                        let diff = max as i32 - min as i32;
                        println!("{subtotals:?}");
                        if diff != 0 {
                            let children = nodes.get(n).unwrap();
                            for c in children {
                                println!("w={}", weights[c]);
                            }
                            println!("diff is {}", diff);
                            todo!();
                        } else {
                            known_totals.insert(n.clone(), min * subtotals.len() + weights[n]);
                        }
                    }
                }
            }
        }
    }
}
