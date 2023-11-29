#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn main() {
    let mut map = HashMap::new();

    for (y, line) in fs::read_to_string("input.txt").unwrap().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != ' ' {
                map.insert((x as i32, y as i32), c);
            }
        }
    }

    let mut pos = map.keys().min_by(|x, y| x.1.cmp(&y.1)).unwrap().to_owned();
    let mut direction = (0, 1);

    let mut seen = HashSet::from([(pos.clone())]);

    let mut steps = 0;
    loop {
        steps += 1;
        let neigh = (pos.0 + direction.0, pos.1 + direction.1);
        // println!("{pos:?}");
        if let Some(c) = map.get(&pos) {
            seen.insert(pos);
            match c {
                'A'..='Z' => {
                    print!("{} (steps={})", c, steps);
                    pos = neigh;
                },
                '|' | '-' => {
                    let nextc = map.get(&neigh).unwrap();
                    if nextc != c && (!nextc.is_alphabetic() && nextc != &'+') {
                        // skip the hole
                        pos = (pos.0 + 2 * direction.0, pos.1 + 2 * direction.1);
                        steps += 1;
                    } else {
                        pos = neigh;
                    }
                },
                '+' => {
                    for m in MOVES {
                        let neigh = (pos.0 + m.0, pos.1 + m.1);
                        if !seen.contains(&neigh) && map.contains_key(&neigh) {
                            direction = m;
                            pos = neigh;
                            break;
                        }
                    }
                },
                _ => {
                    panic!("unexpected char {c}");
                }
            }
        } else {
            steps -= 1;
            break;
        }
    }
    println!();
    println!("steps={steps}");
}
 