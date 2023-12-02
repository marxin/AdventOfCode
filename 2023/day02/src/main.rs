#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};
use itertools::Itertools;

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn main() {
    let maximum = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let mut good = 0;
    let mut suma = 0;

    for (i, line) in fs::read_to_string("input.txt").unwrap().lines().enumerate() {
        let parts: Vec<_> = line.split(':').collect();
        let line = parts[1].trim().to_string();

        let mut good = true;
        for set in line.split(';') {
            let set = set.replace(",", "");
            for (n, color) in set.split_whitespace().tuples() {
                let n = n.parse::<i32>().unwrap();
                let color = color.to_string();
                if n > maximum[&color[..]] {
                    good = false;
                }
            }
        }

        if good {
            suma += i + 1;
        }
    }

    println!("{suma}");
}
