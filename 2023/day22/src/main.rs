#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;

type Point = Vec<i32>;

#[derive(Debug, PartialEq, Eq)]
struct Brick(Point, Point);

fn new_point(part: &str) -> Vec<i32> {
    let mut v = part.split(',').map(|x| x.parse::<i32>().unwrap()).collect_vec();
    v
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

    fn fall_down(& mut self) {        
        self.0[2] -= 1;
        self.1[2] -= 1;
    }

    fn vol(&self) -> usize {
        self.1.iter().zip(&self.0).map(|(&x, y)| x as usize - *y as usize + 1).product()
    }
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let lines = content.lines().collect_vec();

    let mut bricks = Vec::new();
    for (_y, line) in lines.iter().enumerate() {
        let parts = line.split('~').collect_vec();
        bricks.push(Brick(new_point(parts[0]), new_point(parts[1])));
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
  
    assert_eq!(bricks.iter().map(|x| x.vol()).sum::<usize>(), pixels.len());

    let cloned_pixels = pixels.clone();

    let mut counter = 0;
    for (i, candidate) in bricks.iter().enumerate() {
        assert_eq!(cloned_pixels, pixels);
        for p in candidate.get_pixels() {
            pixels.remove(&p);
        }

        if bricks.iter().filter(|&b| b != candidate).all(|b| b.supported(&pixels)) {
            println!("#{} can be removed", i + 1);
            counter += 1;
        }

        for p in candidate.get_pixels() {
            pixels.insert(p);
        }

        assert_eq!(bricks.iter().map(|x| x.vol()).sum::<usize>(), pixels.len());
    }

    println!("max z {:?}", pixels.iter().map(|p| p[2]).max());

    println!("COUNTER = {counter}");
}
