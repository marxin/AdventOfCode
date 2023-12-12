#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

#[derive(Debug)]
struct Arrange
{
    pattern: String,
    groups: Vec<i32>,
}

fn get_groups(pattern: &str) -> Vec<i32> {
    pattern.split('.').filter(|g| g.len() > 0).map(|g| g.len() as i32).collect_vec()
}

fn generate(todo: &[char], prefix: &String, results: & mut Vec<Vec<i32>>) {
    if !todo.is_empty() {
        match todo[0] {
            '?' => {
                let mut prefix = prefix.to_string();
                prefix.push('.');
                generate(&todo[1..], &prefix, results);
                prefix.pop();
                prefix.push('#');
                generate(&todo[1..], &prefix, results);
            },
            _ => {
                let mut prefix = prefix.to_string();
                prefix.push(todo[0]);
                generate(&todo[1..], &prefix, results);
            }
        }
    } else {
        results.push(get_groups(&prefix));
    }
}

fn main() {
    let mut arranges = Vec::new();

    let content = fs::read_to_string("input.txt").unwrap();
    for line in content.lines() {
        let parts = line.split_whitespace().collect_vec();
        arranges.push(Arrange {
            pattern: parts[0].to_string(),
            groups: parts[1].split(',').map(|x| x.parse::<i32>().unwrap()).collect_vec(),
        });
    }

    let mut c = 0;
    for a in arranges {
        let mut results = Vec::new();
        let chars =a.pattern.chars().collect_vec();
        generate(&chars, &String::new(), & mut results);
        for r in results {
            if r == a.groups {
                c += 1;
            }
        }
    }

    println!("{c}");
}
