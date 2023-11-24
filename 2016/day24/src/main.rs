use std::fs;
use std::collections::{HashMap, VecDeque};
use std::ops::RangeFull;

use itertools::Itertools;

const MOVES: [(i32, i32); 4] = [(1i32, 0), (0, 1), (-1, 0), (0, -1)];

#[derive(Debug)]
enum Pixel {
    Wall,
    Empty,
    Robot(usize),
}

fn parse_input() -> HashMap<(usize, usize), Pixel> {
    let mut map = HashMap::new();
    for (y, line) in fs::read_to_string("input.txt").unwrap().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pixel = match c {
                '.' => Pixel::Empty,
                '#' => Pixel::Wall,
                '0'..='9' => Pixel::Robot(c.to_string().parse().unwrap()),
                c => panic!("unexpected char: {c}")
            };
            map.insert((x, y), pixel);
        }
    }
    map
}

fn get_distance(map: &HashMap<(usize, usize), Pixel>, positions: &HashMap<usize, (usize, usize)>, start: usize, end: usize) -> usize {
    assert!(start != end);

    let start_pos = positions[&start];
    let mut queue = VecDeque::from(vec![start_pos]);
    let mut flood = HashMap::new();
    flood.insert(start_pos, 0);

    while let Some(pos) = queue.pop_front() {
        if let Pixel::Robot(n) = map[&pos] {
            if n == end {
                return flood[&pos];
            }
        }
        for m in MOVES {
            let neigh = (pos.0 as i32 + m.0, pos.1 as i32 + m.1);
            let neigh = (neigh.0 as usize, neigh.1 as usize);
            if !flood.contains_key(&neigh) {
                match map[&neigh] {
                    Pixel::Empty | Pixel::Robot(..) => {
                        queue.push_back(neigh);
                        flood.insert(neigh, flood[&pos] + 1);
                    }
                    _ => {}
                }
            }       
        }
    }

    panic!("could not find path");
}

fn main() {
    let map = parse_input();
    let mut digit_positions = HashMap::new();
    for (pos, v) in map.iter() {
        if let Pixel::Robot(n) = v {
            digit_positions.insert(*n, *pos);
        }
    }
    println!("{map:?}");
    println!("{digit_positions:?}");

    let mut best = usize::MAX;
    let N = digit_positions.len();

    let mut distances = HashMap::new();
    for r1 in 0..N {
        for r2 in 0..N {
            if r1 != r2 {
                distances.insert((r1, r2), get_distance(&map, &digit_positions, r1, r2));
            }
        }
    }

    for mut permutation in (1..N).permutations(N - 1) {
        let p = permutation.clone();
        let mut steps = 0;
        let mut robot = 0;
        permutation.push(0);

        while !permutation.is_empty() {
            let end = permutation[0];
            steps += distances[&(robot, end)];
            permutation.remove(0);
            robot = end;
        }
        // println!("{p:?} = {steps}");
        if steps < best {
            best = steps;
             println!("{p:?} = {best}");
        }
    }

    println!("best = {best}");

}
