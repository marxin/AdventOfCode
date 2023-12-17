#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
struct Point(i32, i32);

#[allow(dead_code)]
const MOVES: [Point; 4] = [Point(0, 1), Point(1, 0), Point(0, -1), Point(-1, 0)];

fn dist(p0: &Vec<i32>, p1: &Vec<i32>) -> i32 {
    p0.iter().zip(p1).map(|(x, y)| (x - y).abs()).sum()
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let lines = content.lines().collect_vec();

    let points = lines
        .iter()
        .map(|line| {
            line.trim()
                .split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect_vec();
    let mut mapping = HashMap::new();

    let mut group_id = 0;
    while mapping.len() != points.len() {
        // new group
        let p = points
            .iter()
            .filter(|&x| !mapping.contains_key(x))
            .next()
            .unwrap();
        mapping.insert(p.clone(), group_id);

        loop {
            let mut change = false;
            for p1 in points.iter() {
                if let Some(g) = mapping.get(p1) {
                    if *g == group_id {
                        for p2 in points.iter() {
                            if p1 != p2 && !mapping.contains_key(p2) && dist(p1, p2) <= 3 {
                                change = true;
                                mapping.insert(p2.clone(), group_id);
                            }
                        }
                    }
                }
            }

            if !change {
                break;
            }
        }

        group_id += 1;
    }

    println!("{group_id}");
}
