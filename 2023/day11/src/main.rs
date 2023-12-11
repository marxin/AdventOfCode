#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let mut lines = Vec::new();

    for line in content.lines() {
        lines.push(line.to_string());
    }

    let mut lines2 = lines
        .into_iter()
        .flat_map(|l| {
            if l.contains('#') {
                vec![l]
            } else {
                vec![l.to_owned(), l.to_owned()]
            }
        })
        .collect_vec();

    let mut y = lines2.first().unwrap().len() as i32 - 1;
    while y >= 0 {
        if lines2
            .iter()
            .all(|l| l.chars().nth(y as usize).unwrap() == '.')
        {
            for l in lines2.iter_mut() {
                l.insert(y as usize, '.');
            }
        }

        y -= 1;
    }

    let mut positions = Vec::new();

    for (y, line) in lines2.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                positions.push((x as i32, y as i32));
            }
        }
    }

    let total = positions
        .iter()
        .cartesian_product(&positions)
        .filter_map(|(a, b)| {
            if a == b {
                None
            } else {
                Some((a.0 - b.0).abs() + (a.1 - b.1).abs())
            }
        })
        .sum::<i32>();

    // println!("{}", lines2.join("\n"));
    // println!("{positions:?}");

    println!("{}", total / 2);
}
