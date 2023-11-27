#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn main() {
    let mut programs = VecDeque::from_iter('a'..='p');
    let dances: Vec<_> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|p| p.to_string())
        .collect();
    let mut i = 0;

    let N = 1_000_000_000;

    // period is equal to 48!
    for _ in 0..(N % 48) {
        for token in &dances {
            let parts: Vec<_> = token[1..].split('/').collect();
            match token.chars().next().unwrap() {
                's' => {
                    let n = parts[0].parse::<usize>().unwrap();
                    programs.rotate_right(n);
                }
                'x' => {
                    let x: usize = parts[0].parse().unwrap();
                    let y: usize = parts[1].parse().unwrap();
                    programs.swap(x, y);
                }
                'p' => {
                    let x = programs
                        .iter()
                        .position(|&p| p.to_string() == parts[0])
                        .unwrap();
                    let y = programs
                        .iter()
                        .position(|&p| p.to_string() == parts[1])
                        .unwrap();
                    programs.swap(x, y);
                }
                _ => panic!(),
            }
        }

        let result = String::from_iter(programs.clone());
        println!("{i} = {result}");
    }
}
