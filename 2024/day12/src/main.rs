use std::ops::{Add, Sub};
#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
struct Point(i32, i32);

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

    let map = HashMap::<_, _>::from_iter(lines.iter().enumerate().flat_map(|(y, line)| {
        line.chars()
            .enumerate()
            .map(move |(x, c)| (Point(x as i32, y as i32), c))
    }));

    let mut total = 0;
    let mut visited = HashSet::new();
    for (point, c) in map.iter() {
        if visited.contains(point) {
            continue;
        }
        visited.insert(*point);

        let mut group = HashSet::from([*point]);
        let mut queue = VecDeque::from([*point]);

        while let Some(p) = queue.pop_front() {
            visited.insert(p);

            for m in MOVES.iter() {
                let next = p + *m;
                if map.get(&next) == Some(c) && !group.contains(&next) {
                    queue.push_back(next);
                    group.insert(next);
                }
            }
        }

        let area = group.len();
        let perimeter: usize = group
            .iter()
            .map(|p| {
                MOVES
                    .iter()
                    .filter(|m| {
                        let p2 = *p + **m;
                        map.get(&p2) != Some(c)
                    })
                    .count()
            })
            .sum();
        // println!("{area} {perimeter}");
        total += area * perimeter;
    }

    dbg!(total);

    // dbg!(map);
}
