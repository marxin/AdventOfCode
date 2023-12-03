#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn main() {   
    let content = fs::read_to_string("input.txt").unwrap();
    let mut map = HashMap::new();
    let mut reachable = HashSet::new();

    for (y, line) in content.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                map.insert((x as i32, y as i32), c);
                if !c.is_numeric() {
                    reachable.insert((x as i32, y as i32));
                }
            }
        }
    }

    // propagate reachable locations
    let mut queue = VecDeque::from_iter(reachable.iter().copied());
    while let Some(entry) = queue.pop_back() {
        for x in -1..=1 {
            for y in -1..=1 {
                if x != 0 || y != 0 {
                    let pos = (entry.0 + x, entry.1 + y);
                    if map.contains_key(&pos) && !reachable.contains(&pos) {
                        reachable.insert(pos);
                        queue.push_back(pos);
                    }
                }
            }
        }
    }

    const N: i32 = 1000;
    let mut content = String::new();
    for y in 0..N {
        for x in 0..N {
            let pos = (x, y);
            if reachable.contains(&pos) {
                let value = map[&pos];
                if value.is_digit(10) {
                    content.push(value);
                } else {
                    content.push(',');
                }
            } else {
                content.push(',');
            }
        }
    }

    let s: i64 = content.split(',').filter(|n| !n.is_empty()).map(|n| n.parse::<i64>().unwrap()).sum();
    println!("{s}");
}
