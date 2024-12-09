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

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let lines = content.lines().collect_vec();

    let width = lines.first().unwrap().len() as i32;
    let height = lines.len() as i32;

    let anthenas = lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c != '.' {
                    Some((Point(x as i32, y as i32), c))
                } else {
                    None
                }
            })
        })
        .collect_vec();

    let letters = anthenas.iter().map(|x| x.1).unique().collect_vec();

    let mut antinodes = HashSet::new();

    for c in letters {
        let anthenas = anthenas.iter().filter(|a| a.1 == c).collect_vec();
        for ((p0, _), (p1, _)) in anthenas.iter().cartesian_product(anthenas.iter()) {
            if p0 == p1 {
                continue;
            }
            let d1 = Point(p1.0 - p0.0, p1.1 - p0.1);
            let d2 = Point(p0.0 - p1.0, p0.1 - p1.1);
            //let candadate1 = Point(p1.0 + d1.0, p1.1 + d1.1);
            //let candadate2 = Point(p0.0 + d2.0, p0.1 + d2.1);

            for (c, d) in [(p0, d1), (p1, d2)] {
                let mut c = *c;
                while c.0 >= 0 && c.0 < width && c.1 >= 0 && c.1 < height {
                    antinodes.insert(c);
                    c = Point(c.0 + d.0, c.1 + d.1);
                }
            }
        }
    }

    dbg!(anthenas);
    dbg!(antinodes.len());
}
