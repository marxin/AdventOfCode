#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
struct Point(i64, i64);

#[derive(PartialEq, Eq, Debug)]
struct Line {
    start: Point,
    vector: Point
}

const MIN: f64 = 200000000000000f64;
const MAX: f64 = 400000000000000f64;

//const MIN: f64 = 7f64;
//const MAX: f64 = 27f64;

fn intersection(l0: &Line, l1: &Line) -> Option<(f64, f64)> {
    let p1 = l1.vector.0 * l1.start.1 + l0.start.0 * l1.vector.1 - l1.start.0 * l1.vector.1 - l0.start.1 * l1.vector.0;
    let p2 = (l0.vector.1 * l1.vector.0 - l0.vector.0 * l1.vector.1) as f64;

    let p = p1 as f64 / p2;
    if p < 0f64 {
        return None;
    }

    let q = (l0.start.0 as f64 + p * l0.vector.0 as f64 - l1.start.0 as f64) / l1.vector.0 as f64;
    if q < 0f64 {
        return None;
    }

    if !p.is_nan() {
        Some((l0.start.0 as f64 + p * l0.vector.0 as f64, l0.start.1 as f64 + p * l0.vector.1 as f64))
    } else {
        None
    }
}

fn main() {
    /*
    let lineA = Line {start: Point(19, 13), vector: Point(-2, 1)};
    let lineB = Line {start: Point(18, 19), vector: Point(-1, -1)};
    let lineX = Line {start: Point(17, 18), vector: Point(-1, -1)};
    let lineC = Line { start: Point(20, 25), vector: Point(-2, -2) }; // 20, 25, 34 @ -2, -2,

    println!("{:?}", intersection(&lineA, &lineB));
    println!("{:?}", intersection(&lineA, &lineC));
    println!("{:?}", intersection(&lineB, &lineX));
    println!();
    */

    let content = fs::read_to_string("input.txt").unwrap();
    let lines = content.lines().collect_vec();

    let hailstones = lines.iter().map(|line| {
        // 18, 19, 22 @ -1, -1, -2
        let line = line.replace(" @ ", ", ");
        let parts = line.split(",").map(|x| x.trim().parse::<i64>()).collect_vec();
        Line { start: Point(parts[0].clone().unwrap(), parts[1].clone().unwrap()), vector: Point(parts[3].clone().unwrap(), parts[4].clone().unwrap())}
    }).collect_vec();

    let mut total = 0;

    for h0 in hailstones.iter() {
        for h1 in hailstones.iter() {
            if h0 != h1 {
                let int = intersection(h0, h1);
                // println!("{h0:?} - {h1:?} = {int:?}");
                if let Some(int) = int {
                    if MIN <= int.0 && int.0 <= MAX && MIN <= int.1 && int.1 <= MAX {
                        total += 1;
                    }
                }
            }
        }
    }

    println!("{}", total / 2);

}
