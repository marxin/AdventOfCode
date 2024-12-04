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

fn contains(start: Point, needle: &HashMap<Point, char>, letters: &HashMap<Point, char>) -> bool {
    needle.iter().all(|(pos, c)| {
        letters
            .get(&Point(start.0 + pos.0, start.1 + pos.1))
            .is_some_and(|v| v == c)
    })
}

/*

M.S S.S S.M M.M
.A. .A. .A. .A.
M.S M.M S.M S.S


*/

fn main() {
    let max_needles = [
        HashMap::from([
            (Point(0, 0), 'M'),
            (Point(2, 0), 'S'),
            (Point(1, 1), 'A'),
            (Point(0, 2), 'M'),
            (Point(2, 2), 'S'),
        ]),
        HashMap::from([
            (Point(0, 0), 'S'),
            (Point(2, 0), 'S'),
            (Point(1, 1), 'A'),
            (Point(0, 2), 'M'),
            (Point(2, 2), 'M'),
        ]),
        HashMap::from([
            (Point(0, 0), 'S'),
            (Point(2, 0), 'M'),
            (Point(1, 1), 'A'),
            (Point(0, 2), 'S'),
            (Point(2, 2), 'M'),
        ]),
        HashMap::from([
            (Point(0, 0), 'M'),
            (Point(2, 0), 'M'),
            (Point(1, 1), 'A'),
            (Point(0, 2), 'S'),
            (Point(2, 2), 'S'),
        ]),
    ];

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

    let matches = max_needles
        .iter()
        .cartesian_product(0..width)
        .cartesian_product(0..height)
        .filter(|((needle, x), y)| contains(Point(*x, *y), needle, &letters))
        .count();

    dbg!(matches);
}
