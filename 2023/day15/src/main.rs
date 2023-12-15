#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn get_hash(s: &str) -> u8 {
    s.chars().fold(0, |acc, x| 17 * (acc + x as u8))
}

#[derive(Clone, Debug)]
struct Lens {
    name: String,
    length: u64,
}

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let content = data.trim();
    let mut boxes = Vec::new();
    boxes.resize(256, Vec::<Lens>::new());

    for part in content.split(',') {
        let parts = part.split('=').collect_vec();
        if parts.len() == 2 {
            let h = get_hash(parts.first().unwrap()) as usize;
            let length = parts[1].parse().unwrap();

            if let Some(pos) = boxes[h].iter().position(|x| x.name == parts[0]) {
                boxes[h][pos].length = length;
            } else {
                boxes[h].push(Lens {
                    name: parts[0].to_string(),
                    length,
                })
            }
        } else {
            let remove = part.strip_suffix('-').unwrap();
            let h = get_hash(remove) as usize;
            if let Some(pos) = boxes[h].iter().position(|x| x.name == remove) {
                boxes[h].remove(pos);
            }
        }
    }

    let s = boxes
        .iter()
        .enumerate()
        .map(|(bpos, b)| {
            (bpos + 1) as u64
                * b.iter()
                    .enumerate()
                    .fold(0, |acc, (pos, b)| acc + (pos + 1) as u64 * b.length)
        })
        .sum::<u64>();
    println!("{s}");
}
