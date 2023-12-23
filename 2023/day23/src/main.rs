#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
struct Point(i32, i32);

#[allow(dead_code)]
const MOVES: [Point; 4] = [Point(0, 1), Point(1, 0), Point(0, -1), Point(-1, 0)];

fn is_valid_move(direction: &Point, pixel: &char) -> bool {
    match (pixel, direction) {
        ('>', Point(1, 0)) | ('<', Point(-1, 0)) | ('v', Point(0, 1)) | ('^', Point(0, -1)) => true,
        ('.', ..) => true,
        _ => false,
    }
}

fn visit_all(
    pos: &Point,
    end: &Point,
    mut map: HashMap<Point, char>,
    steps: usize,
    maximum: &mut usize,
) {
    if pos == end {
        if steps > *maximum {
            *maximum = steps;
        }
    }

    let mut candidates = Vec::new();
    for m in MOVES.iter() {
        let p = Point(pos.0 + m.0, pos.1 + m.1);
        if let Some(v) = map.get(&p) {
            if is_valid_move(m, v) {
                candidates.push(p);
            }
        }
    }

    *map.get_mut(&pos).unwrap() = '#';

    if candidates.len() == 1 {
        visit_all(&candidates[0], end, map, steps + 1, maximum);
    } else if candidates.len() > 1 {
        for c in candidates {
            visit_all(&c, end, map.clone(), steps + 1, maximum);
        }
    }
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let lines = content.lines().collect_vec();

    let width = lines.first().unwrap().len() as i32;
    let height = lines.len() as i32;
    assert_eq!(width, height);
    println!("{width}x{height}");

    let mut map = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert(Point(x as i32, y as i32), c);
        }
    }

    let start = map
        .iter()
        .find(|(pos, &c)| pos.1 == 0 && c == '.')
        .unwrap()
        .0;
    let end = map
        .iter()
        .find(|(pos, &c)| pos.1 == height - 1 && c == '.')
        .unwrap()
        .0;
    println!("{start:?} -> {end:?}");

    let mut maximum = 0;
    visit_all(&start, &end, map.clone(), 0, &mut maximum);
    println!("{maximum}");
}
