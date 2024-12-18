use std::ops::{Add, Sub};
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

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

fn visit(pos: Point, map: &HashMap<Point, i32>) -> u64 {
    if map[&pos] == 9 {
        return 1;
    }

    MOVES
        .iter()
        .filter_map(|m| {
            let pos2 = pos + *m;
            if let Some(v2) = map.get(&pos2) {
                if *v2 == map[&pos] + 1 {
                    return Some(visit(pos2, map));
                }
            }
            None
        })
        .sum()
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let lines = content.lines().collect_vec();

    let _width = lines.first().unwrap().len() as i32;
    let _height = lines.len() as i32;

    let mut map = HashMap::new();

    #[allow(unused)]
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert(
                Point(x as i32, y as i32),
                c.to_string().parse::<i32>().unwrap(),
            );
        }
    }

    let mut c = 0;
    for (start, _) in map.iter().filter(|(_, v)| **v == 0) {
        let rating = visit(*start, &map);
        c += rating;
    }

    dbg!(c);
}
