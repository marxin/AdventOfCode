#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn reflection_x(map: &HashSet<(i32, i32)>, W: i32) -> Option<i32> {
    for x in 1..W {
        if map.iter().all(|p| {
                let tocheck =(x - (p.0 - x) - 1, p.1);
                if tocheck.0 < 0 || tocheck.0 >= W {
                    true
                }
                else if map.contains(&tocheck) {
                    true
                } else {
                    false
                }
            }) {
            return Some(x);
        }
    }

    None
}

fn reflection_y(map: &HashSet<(i32, i32)>, H: i32) -> Option<i32> {
    for y in 1..H {
        let tocheck = y.min(H - y);
        if map.iter().all(|p| {
             let tocheck =(p.0, y - (p.1 - y) - 1);
                if tocheck.1 < 0 || tocheck.1 >= H {
                    true
                }
                else if map.contains(&tocheck) {
                    true
                } else {
                    false
                }                
            }) {
            return Some(y);
        }
    }

    None
}

fn main() {
    let mut total = 0;
    let content = fs::read_to_string("input.txt").unwrap();
    let parts = content.split("\n\n").collect_vec();
    

    for part in parts {
        let mut map = HashSet::new();
        let W = part.lines().next().unwrap().chars().count() as i32;
        let H = part.lines().count() as i32;

        // println!("{W}x{H}");

        for (y, line) in part.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    map.insert((x as i32, y as i32));
                }
            }    
        }

        let rx = reflection_x(&map, W);
        let ry = reflection_y(&map, H);
        if let Some(x) = rx {
            total += x;
        } else {
            total += 100 * ry.unwrap();
        }
    }

    println!("{total}");
    
}
