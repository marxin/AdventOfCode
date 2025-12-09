#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};
use std::{
    mem::swap,
    ops::{Add, Mul, Sub},
};

#[allow(unused)]
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
struct Point(i64, i64);

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

impl Mul<i64> for Point {
    type Output = Point;

    fn mul(self, rhs: i64) -> Self::Output {
        Point(self.0 * rhs, self.1 * rhs)
    }
}

#[allow(dead_code)]
const MOVES: [Point; 4] = [Point(0, 1), Point(1, 0), Point(0, -1), Point(-1, 0)];

fn rectangle_size(
    points: &HashSet<Point>,
    xvalues: &Vec<i64>,
    yvalues: &Vec<i64>,
    minx: i64,
    maxx: i64,
    miny: i64,
    maxy: i64,
) -> Option<i64> {
    for x in minx..=maxx {
        for y in miny..=maxy {
            let p = Point(x, y);
            if !points.contains(&p) {
                return None;
            }
        }
    }

    let x = xvalues[maxx as usize + 1] - xvalues[minx as usize];
    let y = yvalues[maxy as usize + 1] - yvalues[miny as usize];
    Some(x * y)
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let lines = content.lines().collect_vec();
    let points = lines
        .iter()
        .map(|l| {
            let parts = l.split_once(',').unwrap();
            Point(
                parts.0.parse::<i64>().unwrap(),
                parts.1.parse::<i64>().unwrap(),
            )
        })
        .collect_vec();

    let mut xvalues = HashSet::new();
    let mut yvalues = HashSet::new();

    let lines: Vec<(_, _)> = points
        .iter()
        .copied()
        .chain([points[0]])
        .tuple_windows()
        .collect_vec();

    for (start, end) in lines.iter() {
        if start.0 == end.0 {
            let (start, end) = if start.1 < end.1 {
                (start, end)
            } else {
                (end, start)
            };
            // vertical line from lower Y to higher
            yvalues.insert(start.1);
            yvalues.insert(start.1 + 1);
            yvalues.insert(end.1);
            yvalues.insert(end.1 + 1);
        } else if start.1 == end.1 {
            let (start, end) = if start.0 < end.0 {
                (start, end)
            } else {
                (end, start)
            };
            // horizontal line from lower X to higher
            xvalues.insert(start.0);
            xvalues.insert(start.0 + 1);
            xvalues.insert(end.0);
            xvalues.insert(end.0 + 1);
        } else {
            unreachable!("wrong lines")
        }
    }

    let xvalues = xvalues.into_iter().sorted().collect_vec();
    let yvalues = yvalues.into_iter().sorted().collect_vec();
    dbg!(&xvalues);
    dbg!(&yvalues);

    let mut points = HashSet::new();

    let mut corners = Vec::new();

    for (start, end) in lines.iter() {
        if start.0 == end.0 {
            let (start, end) = if start.1 < end.1 {
                (start, end)
            } else {
                (end, start)
            };

            // vertical line from lower Y to higher
            let xindex = xvalues.iter().position(|&x| x == start.0).unwrap();
            let mut it = yvalues.iter().skip_while(|&&y| y != start.1);
            let mut s = start.1;
            loop {
                let e = *it.next().unwrap();
                let index = yvalues.iter().position(|&y| y == s).unwrap();
                points.insert(Point(xindex as i64, index as i64));
                if s == end.1 {
                    break;
                }
                s = e;
            }
        } else if start.1 == end.1 {
            let (start, end) = if start.0 < end.0 {
                (start, end)
            } else {
                (end, start)
            };

            // horizontal line from lower X to higher
            let yindex = yvalues.iter().position(|&y| y == start.1).unwrap();
            let mut it = xvalues.iter().skip_while(|&&x| x != start.0);
            let mut s = start.0;
            loop {
                let e = *it.next().unwrap();
                let index = xvalues.iter().position(|&x| x == s).unwrap();
                points.insert(Point(index as i64, yindex as i64));
                if s == end.0 {
                    break;
                }
                s = e;
            }
        } else {
            unreachable!("wrong lines")
        }

        let x = xvalues.iter().position(|&x| x == start.0).unwrap() as i64;
        let y = yvalues.iter().position(|&y| y == start.1).unwrap() as i64;
        corners.push(Point(x, y));
    }

    dbg!(xvalues.len());
    dbg!(yvalues.len());

    // let s = Point(2, 2);
    let s = Point(250, 400);
    let mut worklist = vec![s];

    while let Some(pos) = worklist.pop() {
        for m in MOVES.iter() {
            let pos2 = pos + *m;
            if !points.contains(&pos2) {
                points.insert(pos2);
                worklist.push(pos2);
            }
        }
    }

    for y in 0..yvalues.len() as i64 {
        for x in 0..xvalues.len() as i64 {
            let p = Point(x, y);
            if corners.contains(&p) {
                print!("X");
            } else if points.contains(&p) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!()
    }

    let mut best = 0;
    for (start, end) in corners.iter().cartesian_product(corners.iter()) {
        let minx = start.0.min(end.0);
        let maxx = start.0.max(end.0);
        let miny = start.1.min(end.1);
        let maxy = start.1.max(end.1);

        if let Some(b) = rectangle_size(&points, &xvalues, &yvalues, minx, maxx, miny, maxy) {
            best = best.max(b);
        }
    }

    dbg!(best);
}
