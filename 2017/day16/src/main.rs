#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn main() {
    let mut programs = VecDeque::from_iter('a'..='p');
    println!("{programs:?}");
    for token in fs::read_to_string("input.txt").unwrap().lines().next().unwrap().split(',') {
        let parts: Vec<_> = token[1..].split('/').collect();
        match token.chars().next().unwrap() {
            's' => {
                let n = parts[0].parse::<usize>().unwrap();
                programs.rotate_right(n);
            },
            'x' => {
                let x: usize = parts[0].parse().unwrap();
                let y: usize = parts[1].parse().unwrap();
                programs.swap(x, y);
            },
            'p' => {
                let x = programs.iter().position(|&p| p.to_string() == parts[0]).unwrap();
                let y = programs.iter().position(|&p| p.to_string() == parts[1]).unwrap();
                programs.swap(x, y);
            },
            _ => panic!()
        }            
    }

    println!("{programs:?}");
    println!("{}", String::from_iter(programs));
}
