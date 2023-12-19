#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
struct Point(i32, i32);

#[allow(dead_code)]
const MOVES: [Point; 4] = [Point(1, 0), Point(0, 1), Point(-1, 0), Point(0, -1)];
const MOVE_LETTERS: [&str; 4] = ["U", "R", "D", "L"];

#[derive(Debug)]
struct Rect {
    start: Point,
    end: Point,
}

impl Rect {
    fn contains(&self, other: &Rect) -> bool {
        assert!(self.start.0 < self.end.0);
        assert!(self.start.1 < self.end.1);

        self.start.0 <= other.start.0
            && other.start.0 <= self.end.0
            && self.start.0 <= other.end.0
            && other.end.0 <= self.end.0
            && self.start.1 <= other.start.1
            && other.start.1 <= self.end.1
            && self.start.1 <= other.end.1
            && other.end.1 <= self.end.1
    }

    fn area(&self) -> u64 {
        (self.end.0 - self.start.0) as u64 * (self.end.1 - self.start.1) as u64
    }
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let mut pos = Point(0, 0);
    let mut points = Vec::new();
    let mut area: u64 = 0;
    let mut boundary = Vec::new();

    for line in content.lines() {
        let parts = line.split_whitespace().collect_vec();
        let hex = &parts[2][2..parts[2].len() - 2];
        // println!("{hex:?}");
        let steps = i32::from_str_radix(hex, 16).unwrap();

        // #70c710 = R 461937
        let mindex = i32::from_str_radix(
            parts[2].chars().rev().nth(1).unwrap().to_string().as_str(),
            16,
        )
        .unwrap();

        // part 1
        //let mindex = MOVE_LETTERS.iter().position(|&x| x == parts[0]).unwrap();
        //let steps = parts[1].parse::<i32>().unwrap();

        let m = MOVES[mindex as usize];

        let next = Point(pos.0 + steps * m.0, pos.1 + steps * m.1);
        let mut rect = match mindex {
            0 | 1 => Rect {
                start: pos,
                end: next,
            },
            2 | 3 => Rect {
                start: next,
                end: pos,
            },
            _ => panic!(),
        };

        rect = Rect {
            start: rect.start,
            end: Point(rect.end.0 + 1, rect.end.1 + 1),
        };

        area += steps as u64;
        boundary.push(rect);
        pos = next;

        points.push(pos.to_owned());
    }

    assert_eq!(Point(0, 0), pos);

    let xvalues = points
        .iter()
        .map(|Point(x, _)| x)
        .map(|&x| vec![x - 1, x, x + 1])
        .flatten()
        .sorted()
        .unique()
        .collect_vec();
    let yvalues = points
        .iter()
        .map(|Point(_, y)| y)
        .map(|&y| vec![y - 1, y, y + 1])
        .flatten()
        .sorted()
        .unique()
        .collect_vec();

    // println!("{xvalues:?}");
    // println!("{yvalues:?}");
    println!("{} x {}", xvalues.len(), yvalues.len());

    assert!(Rect {
        start: Point(0, 0),
        end: Point(10, 1)
    }
    .contains(&Rect {
        start: Point(0, 0),
        end: Point(1, 0)
    }));
    assert!(!Rect {
        start: Point(0, 0),
        end: Point(10, 1)
    }
    .contains(&Rect {
        start: Point(-1, 0),
        end: Point(1, 0)
    }));
    assert!(Rect {
        start: Point(0, 0),
        end: Point(10, 100)
    }
    .contains(&Rect {
        start: Point(2, 2),
        end: Point(10, 100)
    }));

    let x = xvalues.iter().position(|v| v == &0).unwrap() + 1;
    let y = yvalues.iter().position(|v| v == &0).unwrap() + 1;

    let pixel = Point(x as i32, y as i32);
    println!("{pixel:?}");

    let mut seen = HashSet::new();
    let mut queue = VecDeque::from([pixel.clone()]);

    while let Some(pixel) = queue.pop_front() {
        if seen.contains(&pixel) {
            continue;
        }

        let rect = Rect {
            start: Point(xvalues[pixel.0 as usize], yvalues[pixel.1 as usize]),
            end: Point(
                xvalues[(pixel.0 + 1) as usize],
                yvalues[(pixel.1 + 1) as usize],
            ),
        };

        // println!("{rect:?}");
        area += rect.area();
        seen.insert(pixel);

        for m in MOVES {
            let next_pixel = Point(pixel.0 + m.0, pixel.1 + m.1);
            if !seen.contains(&next_pixel) {
                let next_rect = Rect {
                    start: Point(
                        xvalues[next_pixel.0 as usize],
                        yvalues[next_pixel.1 as usize],
                    ),
                    end: Point(
                        xvalues[(next_pixel.0 + 1) as usize],
                        yvalues[(next_pixel.1 + 1) as usize],
                    ),
                };
                if boundary.iter().all(|b| !b.contains(&next_rect)) {
                    queue.push_back(next_pixel);
                }
            }
        }
    }

    println!("{area}");

    /*
    let mut queue = VecDeque::from(vec![Point(1, 0)]);
    let mut inqueue = HashSet::new();
    inqueue.insert(Point(0, 0));

    let mut seen = HashSet::new();

    let mut x = 0;
    while let Some(pos) = queue.pop_front() {
        x += 1;
        if !seen.contains(&pos) {
            seen.insert(pos);
        }

        for m in MOVES {
            let next = Point(pos.0 + m.0, pos.1 + m.1);
            if !mapa.contains_key(&next) && !inqueue.contains(&next){
                queue.push_back(next.clone());
                inqueue.insert(next);
            }
        }
    }

    println!("{} + {} = {}", mapa.len(), seen.len(), mapa.len() + seen.len());
    */
}
