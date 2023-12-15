#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

use regex::Regex;

#[allow(unused)]
use itertools::Itertools;

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

// pos=<0,0,0>, r=4

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Bot {
    pos: Vec<i32>,
    radius: i32,
}

impl Bot {
    fn neighbors(&self, bots: &HashSet<Bot>) -> usize {
        bots.iter()
            .filter(|&other| {
                let distance = (0..3)
                    .map(|i| (self.pos[i] - other.pos[i]).abs())
                    .sum::<i32>();
                distance <= self.radius
            })
            .count()
    }
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let lines = content.lines().collect_vec();

    let _width = lines.first().unwrap().len() as i32;
    let _height = lines.len() as i32;
    let mut bots = Vec::new();

    let pos_regex = Regex::new(r".*<(.*)>.*").unwrap();
    for (_y, line) in lines.iter().enumerate() {
        let pos = pos_regex.captures(line).unwrap()[1]
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect_vec();
        let radius = line.split('=').last().unwrap().parse::<i32>().unwrap();
        bots.push(Bot { pos, radius });
    }

    let bots_set: HashSet<Bot> = HashSet::from_iter(bots.clone());
    bots.sort_by(|a, b| b.neighbors(&bots_set).cmp(&a.neighbors(&bots_set)));

    // println!("{bots:?}");
    println!("{}", bots[0].neighbors(&bots_set));
}
