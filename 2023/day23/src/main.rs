#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
struct Point(i32, i32);

#[allow(dead_code)]
const MOVES: [Point; 4] = [Point(0, 1), Point(1, 0), Point(0, -1), Point(-1, 0)];

fn visit(
    pos: &Point,
    end: &Point,
    cross_roads: &HashMap<Point, Vec<(Point, usize)>>,
    seen: &mut HashSet<Point>,
    steps: usize,
    maximum: &mut usize,
) {
    if pos == end {
        if steps > *maximum {
            *maximum = steps;
            println!("new best steps: {steps}");
        }
        return;
    }

    for (next, distance) in &cross_roads[pos] {
        if !seen.contains(&next) {
            seen.insert(next.clone());
            visit(&next, end, cross_roads, seen, steps + distance, maximum);
            seen.remove(&next);
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
            map.insert(Point(x as i32, y as i32), if c == '#' { c } else { '.' });
        }
    }

    let mut cross_roads = HashMap::new();
    for (p, c) in map.iter() {
        if c == &'.' {
            let mut neighbors = 0;
            for m in MOVES {
                let next = Point(p.0 + m.0, p.1 + m.1);
                if let Some(v) = map.get(&next) {
                    if v == &'.' {
                        neighbors += 1;
                    }
                }
            }

            if neighbors > 2 {
                cross_roads.insert(p.clone(), vec![]);
            }
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

    cross_roads.insert(start.clone(), vec![]);
    cross_roads.insert(end.clone(), vec![]);

    for c in cross_roads.clone().keys() {
        let mut queue = VecDeque::from_iter(vec![(c.clone(), 0)]);
        let mut seen = HashSet::new();

        while let Some((point, steps)) = queue.pop_front() {
            if seen.contains(&point) {
                continue;
            }
            seen.insert(point.clone());

            if point != *c && cross_roads.contains_key(&point) {
                cross_roads.get_mut(&c).unwrap().push((point, steps));
            } else {
                for m in MOVES {
                    let next = Point(point.0 + m.0, point.1 + m.1);
                    if let Some(v) = map.get(&next) {
                        if v == &'.' {
                            queue.push_back((next, steps + 1));
                        }
                    }
                }
            }
        }
    }

    println!("{cross_roads:?}");

    let mut maximum = 0;
    visit(
        &start,
        &end,
        &cross_roads,
        &mut HashSet::from_iter(vec![start.clone()]),
        0,
        &mut maximum,
    );
}
