#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
struct Point(i32, i32);

#[allow(dead_code)]
const MOVES: [Point; 4] = [Point(0, 1), Point(1, 0), Point(0, -1), Point(-1, 0)];

fn step(map: &HashSet<(i32, i32)>, positions: HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let mut next = HashSet::new();

    for p in positions.iter() {
        for m in MOVES {
            let next_pos = (p.0 + m.0, p.1 + m.1);
            if map.contains(&next_pos) {
                next.insert(next_pos);
            }
        }
    }

    next
}

fn print(
    map: &HashSet<(i32, i32)>,
    rocks: &HashSet<(i32, i32)>,
    N: i32,
    K: i32,
    start: &(i32, i32),
) {
    for yy in -K..=K {
        for y in 0..N {
            for xx in -K..=K {
                for x in 0..N {
                    let p = (x + xx * N, y + yy * N);
                    if p == *start {
                        print!("S");
                    } else if map.contains(&p) {
                        print!("O");
                    } else if rocks.contains(&p) {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
            }
            println!();
        }
    }
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let lines = content.lines().collect_vec();

    let width = lines.first().unwrap().len() as i32;
    let height = lines.len() as i32;
    let N = width;
    assert_eq!(width, height);
    println!("{width}x{height}");

    let mut start = None;
    let mut map = HashSet::new();
    let mut rocks = HashSet::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = (x as i32, y as i32);
            match c {
                '#' => {
                    rocks.insert(pos);
                }
                '.' => {
                    map.insert(pos);
                }
                'S' => {
                    start = Some(pos);
                    map.insert(pos);
                }
                _ => todo!(),
            }
        }
    }

    let mut supermap = HashSet::new();
    let mut superrock = HashSet::new();

    // copy map
    let K = 8;
    for x in -K..K {
        for y in -K..K {
            for p in map.iter() {
                supermap.insert((x * N + p.0, y * N + p.1));
            }
        }
    }

    for x in -K..K {
        for y in -K..K {
            for p in rocks.iter() {
                superrock.insert((x * N + p.0, y * N + p.1));
            }
        }
    }

    let mut positions: HashSet<(i32, i32)> = HashSet::from_iter(vec![start.unwrap()]);
    print(&positions, &superrock, N, 1, &start.unwrap());
    println!();

    let mut last = 0;

    let guess = |D| {
        5764u64
            + 5755
            + 5756
            + 5747
            + 2 * D * 964
            + D * 965
            + D * 984
            + (D * D) * 7650
            + (D - 1) * (D - 1) * 7637
            + (D - 1) * (6703 + 6690 + 6694 + 6698)
    };
    println!("Guess for 202300: {}", guess(202300));

    let mut D = 0u64;

    for i in 1..2000 {
        positions = step(&supermap, positions);

        if i >= 65 && (i - 65) % (2 * 131) == 0 {
            println!("X={D} (total: {})", positions.len());

            println!("guess = {}", guess(D));

            for y in -K..K {
                for x in -K..K {
                    let mut pixels = 0;
                    for p in positions.iter() {
                        if x * N <= p.0 && p.0 < (x + 1) * N && y * N <= p.1 && p.1 < (y + 1) * N {
                            pixels += 1;
                        }
                    }
                    if pixels != 0 {
                        print!("{pixels:05} ");
                    } else {
                        print!("      ");
                    }
                }
                println!();
            }
            D += 2;
        }
    }

    //println!("{}", positions.len());
}
