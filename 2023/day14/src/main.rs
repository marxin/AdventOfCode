#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs, mem};

#[allow(unused)]
use itertools::Itertools;

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

#[allow(unused)]
fn print_map(map: &HashMap<(i32, i32), char>, dim: i32) {
    for y in 0..dim {
        for x in 0..dim {
            if let Some(v) = map.get(&(x, y)) {
                print!("{v}");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn rotate(map: HashMap<(i32, i32), char>, dim: i32) -> HashMap<(i32, i32), char> {
    map.into_iter()
        .map(|(pos, c)| ((dim - pos.1 - 1, pos.0), c))
        .collect()
}

fn score(map: &HashMap<(i32, i32), char>, dim: i32) -> i32 {
    map.iter()
        .filter_map(|(pos, &c)| if c == 'O' { Some(dim - pos.1) } else { None })
        .sum()
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let lines = content.lines().collect_vec();

    let width = lines.first().unwrap().len() as i32;
    let height = lines.len() as i32;
    let dim = width;
    assert_eq!(width, height);
    println!("{width} x {height}");

    let mut points = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                points.insert((x as i32, y as i32), c);
            }
        }
    }

    let mut cache: HashMap<Vec<((i32, i32), char)>, (i32, i32)> = HashMap::new();

    let mut i = 0;
    const TIMES: i32 = 1_000_000_000;

    while i != TIMES {
        for _ in 0..4 {
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
            // print_map(&points, dim);
            points = rotate(points, dim);
        }
        let score = score(&points, dim);
        println!("{} = {}", i, score);

        let key = points.clone().into_iter().sorted().collect_vec();
        if let Some(v) = cache.get(&key) {
            let cycle = i - (*v).0;
            let cycles = (TIMES - i) / cycle;
            if cycles > 0 {
                println!("cycle={cycle}, {cycles}");
                i += cycles * cycle;
            }
        } else {
            cache.insert(key, (i, score));
        }

        i += 1;
    }

    let score = score(&points, dim);
    println!("{score}");
}
