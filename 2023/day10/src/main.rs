#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;

/*
| is a vertical pipe connecting north and south.
- is a horizontal pipe connecting east and west.
L is a 90-degree bend connecting north and east.
J is a 90-degree bend connecting north and west.
7 is a 90-degree bend connecting south and west.
F is a 90-degree bend connecting south and east.
 */

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn main() {
    let pipes = [
        ('|', (0, 1), (0, 1)),
        ('|', (0, -1), (0, -1)),
        ('-', (1, 0), (1, 0)),
        ('-', (-1, 0), (-1, 0)),
        ('L', (0, 1), (1, 0)),
        ('L', (-1, 0), (0, -1)),
        ('J', (0, 1), (-1, 0)),
        ('J', (1, 0), (0, -1)),
        ('7', (1, 0), (0, 1)),
        ('7', (0, -1), (-1, 0)),
        ('F', (0, -1), (1, 0)),
        ('F', (-1, 0), (0, 1)),
    ];

    let mut map = HashMap::new();

    let content = fs::read_to_string("input.txt").unwrap();
    for (y, line) in content.lines().enumerate() {
        for(x, c) in line.chars().enumerate() {
            map.insert((x as i32, y as i32), c);
        }
    }

    println!("{map:?}");

    let start = map.iter().find(|(_, &v)| v == 'S').unwrap().0.to_owned();
    println!("START: {start:?}");

    let mut orientation = (0, 1);
    let mut pos = start;
    let mut steps = 0;

    while pos != start || steps == 0 {
        let nextpos = (pos.0 + orientation.0, pos.1 + orientation.1);
        if map[&nextpos] == 'S' {
            break;
        }

        let next_orient = pipes.iter().find(|x| x.0 == map[&nextpos] && x.1 == orientation).unwrap().2;
        pos = nextpos;
        orientation = next_orient;
        steps += 1;
        println!("{pos:?} {orientation:?}");
    }
    
    println!("{}", (steps + 1) / 2);

}
