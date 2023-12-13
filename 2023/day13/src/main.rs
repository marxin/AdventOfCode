#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn reflection_x(map: &HashSet<(i32, i32)>, W: i32) -> Vec<i32> {
    let mut results = Vec::new();

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
            results.push(x);
        }
    }

    results
}

fn reflection_y(map: &HashSet<(i32, i32)>, H: i32) -> Vec<i32> {
    let mut results = Vec::new();

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
            results.push(y);
        }
    }

    results
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
        assert!(rx.len() <= 1);
        let ry = reflection_y(&map, H);
        assert!(ry.len() <= 1);
        // println!("has: {rx:?} {ry:?}");

        let mut done = false;
        'outer: for x in 0..W {
            for y in 0..H {
                let mut map2 = map.clone();
                let p = (x, y);
                if map2.contains(&p) {
                    map2.remove(&p);
                } else {
                    map2.insert(p);
                }

                let mut rx2 = reflection_x(&map2, W);
                let mut ry2 = reflection_y(&map2, H);
                if !rx2.is_empty() && rx != rx2 {
                    if !rx.is_empty() {
                        rx2.retain(|v| v != &rx[0]);
                    }
                    total += rx2[0];
                    done = true;
                    break 'outer;
                }
                else if !ry2.is_empty() && ry != ry2 {
                    if !ry.is_empty() {
                        ry2.retain(|v| v != &ry[0]);
                    }
                    total += 100 * ry2[0];
                    done = true;
                    break 'outer;
                }              
            }
        }

        assert_eq!(true, done);


        
    }

    println!("{total}");
    
}
