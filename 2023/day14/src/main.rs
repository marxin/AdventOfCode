#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn print_map(map: &HashMap<(i32, i32), char>, width: i32, height: i32) {
    for y in 0..height {
        for x in 0..width {
            if let Some(v) = map.get(&(x, y)) {
                print!("{v}");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let lines = content.lines().collect_vec();

    let width = lines.first().unwrap().len() as i32;
    let height = lines.len() as i32;
    println!("{width} x {height}");

    let mut points = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                points.insert((x as i32, y as i32), c);
            }
        }
    }

    loop {
        let mut change = false;

        let mut next_points = HashMap::new();
        for (k, v) in points.iter() {
            let pos = (k.0, k.1 - 1);
            if *v == 'O' && pos.1 >= 0 && pos.1 < height && !points.contains_key(&pos) {
                next_points.insert(pos, *v);
                change = true;
            } else {
                next_points.insert(*k, *v);
            }
        }
        points = next_points;

        if !change {
            break;
        }
    }

    print_map(&points, width, height);

    let score: i32 = points.iter().filter_map(|(pos, &c)| {
        if c == 'O' {
            Some(height - pos.1)
        } else {
            None
        }
    }).sum();

    println!("{score}");
}
