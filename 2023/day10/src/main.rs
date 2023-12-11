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

const PIXEL: i32 = 3;

fn print_map(width: i32, height: i32, map: &HashSet<(i32, i32)>) {
    for y in 0..PIXEL * height {
        for x in 0..PIXEL * width {
            if map.contains(&(x as i32, y as i32)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn main() {
    let pixelization = HashMap::from([
        ('|', HashSet::from([(1, 0), (1, 1), (1, 2)])),
        ('-', HashSet::from([(0, 1), (1, 1), (2, 1)])),
        ('L', HashSet::from([(1, 0), (1, 1), (2, 1)])),
        ('J', HashSet::from([(1, 0), (1, 1), (0, 1)])),
        ('J', HashSet::from([(0, 1), (1, 1), (1, 0)])),
        ('7', HashSet::from([(0, 1), (1, 1), (1, 2)])),
        ('F', HashSet::from([(1, 2), (1, 1), (2, 1)])),
        ('S', HashSet::from([(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)])),
    ]);

    let mut map = HashSet::new();

    let content = fs::read_to_string("input.txt").unwrap();
    let width = content.lines().next().unwrap().len() as i32;
    let height = content.lines().count() as i32;

    for (y, line) in content.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if let Some(pixels) = pixelization.get(&c) {
                for (px, py) in pixels {
                    map.insert((PIXEL * x as i32 + px, PIXEL * y as i32 + py));
                }
            }
        }
    }

    print_map(width, height, &map);
}
