#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};
use std::{
    hash::Hash,
    ops::{Add, Sub},
};

#[allow(unused)]
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
struct Point(i64, i64, i64);

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

fn dist(a: &Point, b: &Point) -> f32 {
    let dx2 = (a.0 - b.0).pow(2) as f32;
    let dy2 = (a.1 - b.1).pow(2) as f32;
    let dy = (dx2 + dy2).sqrt();

    let dz2 = (a.2 - b.2).pow(2) as f32;
    (dy * dy + dz2).sqrt()
}

fn is_one_component(points: &Vec<Point>, edges: &HashMap<Point, Vec<Point>>) -> bool {
    let mut worklist = vec![points[0]];
    let mut seen = HashSet::new();

    while let Some(item) = worklist.pop() {
        if seen.contains(&item) {
            continue;
        }
        seen.insert(item);
        if let Some(neighbors) = edges.get(&item) {
            for n in neighbors {
                worklist.push(*n);
            }
        }
    }

    seen.len() == points.len()
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let points = content
        .lines()
        .map(|line| {
            let parts = line
                .split(',')
                .map(|x| x.parse::<i64>().unwrap())
                .collect_vec();
            Point(parts[0], parts[1], parts[2])
        })
        .collect_vec();

    let mut distances = Vec::new();
    for (p1, p2) in points.iter().cartesian_product(points.iter()) {
        if p1 != p2 {
            distances.push((dist(p1, p2), (p1, p2)));
        }
    }
    distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let distances = distances.chunks(2).map(|chunk| chunk[0]).collect_vec();

    let mut edges = HashMap::new();
    for (_, (&p1, &p2)) in distances.iter().copied() {
        edges
            .entry(p1)
            .and_modify(|v: &mut Vec<Point>| v.push(p2))
            .or_insert_with(|| vec![p2]);
        edges
            .entry(p2)
            .and_modify(|v: &mut Vec<Point>| v.push(p1))
            .or_insert_with(|| vec![p1]);

        if is_one_component(&points, &edges) {
            println!("{}", p1.0 * p2.0);
            break;
        }
    }
}
