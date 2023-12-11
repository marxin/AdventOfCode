use std::f32::consts::PI;
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

fn print_map(width: i32, height: i32, map: &HashSet<(i32, i32)>, outside: &HashSet<(i32, i32)>) {
    for y in 0..PIXEL * height {
        for x in 0..PIXEL * width {
            if map.contains(&(x as i32, y as i32)) {
                print!("#");
            }
            else if outside.contains(&(x as i32, y as i32)) {
                print!("O");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn is_inside(pos: (i32, i32), map: &HashSet<(i32, i32)>, outside: &HashSet<(i32, i32)>) -> bool {
    for y in 0..PIXEL {
        for x in 0..PIXEL {
            let p = (pos.0 * PIXEL + x, pos.1 * PIXEL + y);
            if map.contains(&p) || outside.contains(&p) {
                return false;
            }
        }
    }

    true
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

    let mut start = None;

    for (y, line) in content.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if let Some(pixels) = pixelization.get(&c) {
                if c == 'S' {
                    start = Some((x as i32 * PIXEL + 1, y as i32 * PIXEL + 1));
                }
                for (px, py) in pixels {
                    map.insert((PIXEL * x as i32 + px, PIXEL * y as i32 + py));
                }
            }
        }
    }

    println!("START: {start:?}");
    let mut queue = VecDeque::from([start.unwrap()]);
    let mut pipes = HashSet::new();

    while let Some(p) = queue.pop_back() {
        pipes.insert(p.to_owned());
        for m in MOVES {
            let next = (p.0 + m.0, p.1 + m.1);
            if map.contains(&next) && !pipes.contains(&next) {
                queue.push_back(next);
            }
        }
    }

    print_map(width, height, &pipes, &HashSet::new());
    map = pipes;

    let mut queue = VecDeque::new();
    let mut outside = HashSet::new();

    for y in 0..PIXEL * height {
        for x in 0..PIXEL * width {
            queue.push_back((x, 0));
            queue.push_back((x, PIXEL * height - 1));
            queue.push_back((0, y));
            queue.push_back((PIXEL * width - 1, y));
        }
    }

    while let Some(p) = queue.pop_back() {
        outside.insert(p);
        for m in MOVES {
            let next = (p.0 + m.0, p.1 + m.1);
            if next.0 >=0 && next.0 < PIXEL * width && next.1 >= 0 && next.1 < PIXEL * height && !outside.contains(&next) && !map.contains(&next) {
                queue.push_back(next);
            }
        }
    }

    print_map(width, height, &map, &outside);

    let inside = (0..width).cartesian_product(0..height).filter(|(x, y)| is_inside((*x, *y), &map, &outside)).count();
    println!("{inside}");
}
