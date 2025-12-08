use std::ops::{Add, Sub};
#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

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
    for (_, (&p1, &p2)) in distances.iter().take(1000).copied() {
        edges
            .entry(p1)
            .and_modify(|v: &mut Vec<Point>| v.push(p2))
            .or_insert_with(|| vec![p2]);
        edges
            .entry(p2)
            .and_modify(|v: &mut Vec<Point>| v.push(p1))
            .or_insert_with(|| vec![p1]);
    }

    let mut seen: HashSet<Point> = HashSet::new();
    let mut sizes = Vec::new();
    for p in points {
        if seen.contains(&p) {
            continue;
        }
        let mut size = 0;
        let mut worklist = vec![p];
        while let Some(item) = worklist.pop() {
            if seen.contains(&item) {
                continue;
            }
            seen.insert(item);
            size += 1;
            if let Some(neighbors) = edges.get(&item) {
                for n in neighbors {
                    worklist.push(*n);
                }
            }
        }

        if size > 1 {
            sizes.push(size);
        }
    }

    sizes.sort();
    let best_3 = sizes.iter().rev().take(3).collect_vec();
    println!("{}", best_3[0] * best_3[1] * best_3[2]);
}
