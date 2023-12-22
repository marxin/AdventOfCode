#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;

type Point = Vec<i32>;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Brick(Point, Point, bool);

fn new_point(part: &str) -> Vec<i32> {
    part.split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect_vec()
}

impl Brick {
    fn get_pixels(&self) -> Vec<Point> {
        for i in 0..3 {
            assert!(self.0[i] <= self.1[i]);
        }

        let mut pixels = Vec::new();
        for x in self.0[0]..=self.1[0] {
            for y in self.0[1]..=self.1[1] {
                for z in self.0[2]..=self.1[2] {
                    pixels.push(vec![x, y, z]);
                }
            }
        }

        pixels
    }

    fn supported(&self, pixels: &HashSet<Vec<i32>>) -> bool {
        if self.0[2] == 1 {
            return true;
        }

        for x in self.0[0]..=self.1[0] {
            for y in self.0[1]..=self.1[1] {
                let p = vec![x, y, self.0[2] - 1];
                if pixels.contains(&p) {
                    return true;
                }
            }
        }

        false
    }

    fn fall_down(&mut self) {
        self.0[2] -= 1;
        self.1[2] -= 1;
    }
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let lines = content.lines().collect_vec();

    let mut bricks = Vec::new();
    for (_y, line) in lines.iter().enumerate() {
        let parts = line.split('~').collect_vec();
        bricks.push(Brick(new_point(parts[0]), new_point(parts[1]), false));
    }

    let mut pixels = HashSet::new();
    for brick in bricks.iter_mut() {
        // verify there's not intersection
        for p in brick.get_pixels() {
            assert!(!pixels.contains(&p));
            pixels.insert(p);
        }
    }

    let mut change = true;
    while change {
        change = false;

        for b in bricks.iter_mut() {
            if !b.supported(&pixels) {
                change = true;
                for p in b.get_pixels() {
                    pixels.remove(&p);
                }

                b.fall_down();
                for p in b.get_pixels() {
                    pixels.insert(p);
                }
            }
        }
    }

    let mut totalus = 0;

    let mut counter = 0;
    for (i, candidate) in bricks.iter().enumerate() {
        let mut cloned = pixels.clone();
        for p in candidate.get_pixels() {
            cloned.remove(&p);
        }

        let mut candidates = bricks
            .iter()
            .filter(|&x| x != candidate)
            .cloned()
            .collect_vec();
        loop {
            let mut change = false;
            for f in candidates.iter_mut() {
                if !f.2 && !f.supported(&cloned) {
                    f.2 = true;
                    for p in f.get_pixels() {
                        cloned.remove(&p);
                    }
                    change = true;
                }
            }
            if !change {
                break;
            }
        }

        let fallen = candidates.iter().filter(|&x| x.2).count();
        totalus += fallen;

        if fallen > 0 {
            // println!("#{} fall count: {}", i + 1, fallen);
            counter += 1;
        }

        for p in candidate.get_pixels() {
            cloned.insert(p);
        }
    }

    println!("max z {:?}", pixels.iter().map(|p| p[2]).max());

    println!("COUNTER = {counter}");
    println!("TOTALUS = {totalus}");
}
