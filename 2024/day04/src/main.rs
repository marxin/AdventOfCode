#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
struct Point(i32, i32);

#[allow(dead_code)]
const MOVES: [Point; 4] = [Point(0, 1), Point(1, 0), Point(0, -1), Point(-1, 0)];

#[allow(dead_code)]
const MOVES_WITH_DIAGONAL: [Point; 8] = [
    Point(0, 1),
    Point(1, 0),
    Point(0, -1),
    Point(-1, 0),
    Point(1, 1),
    Point(1, -1),
    Point(-1, 1),
    Point(-1, -1),
];

const NEEDLE: &str = "XMAS";

fn contains(start: Point, dir: &Point, letters: &HashMap<Point, char>) -> bool {
    NEEDLE.chars().enumerate().all(|(i, c)| {
        letters
            .get(&Point(
                start.0 + i as i32 * dir.0,
                start.1 + i as i32 * dir.1,
            ))
            .is_some_and(|v| v == &c)
    })
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let lines = content.lines().collect_vec();

    let width = lines.first().unwrap().len() as i32;
    let height = lines.len() as i32;

    let mut letters = HashMap::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            letters.insert(Point(x as i32, y as i32), c);
        }
    }

    let matches = MOVES_WITH_DIAGONAL
        .iter()
        .cartesian_product(0..width)
        .cartesian_product(0..height)
        .filter(|((dir, x), y)| contains(Point(*x, *y), dir, &letters))
        .count();

    dbg!(matches);
}
