#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs, ops};

#[derive(Debug, Clone)]
struct Point(i64, i64, i64);

impl ops::Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl From<Vec<i64>> for Point {
    fn from(value: Vec<i64>) -> Self {
        assert_eq!(3, value.len());
        Self(value[0], value[1], value[2])
    }
}

impl Point {
    fn distance(&self) -> i64 {
        self.0.abs() + self.1.abs() + self.2.abs()
    }
}

#[derive(Debug)]
struct Particle {
    id: usize,
    pos: Point,
    vel: Point,
    acc: Point
}

impl Particle {
    fn step(& mut self) {
        self.vel = self.vel.clone() + self.acc.clone();
        self.pos = self.pos.clone() + self.vel.clone();
    }
}

impl Particle {
    fn new(id: usize, line: &str) -> Self {
        // p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>
        // p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>
        let parts: Vec<_> = line.split(", ").collect();
        assert_eq!(3, parts.len());
        let pos: Vec<_> = parts[0].strip_prefix("p=<").unwrap().strip_suffix(">").unwrap().split(',').map(|c| c.parse::<i64>().unwrap()).collect();
        let vel: Vec<_> = parts[1].strip_prefix("v=<").unwrap().strip_suffix(">").unwrap().split(',').map(|c| c.parse::<i64>().unwrap()).collect();
        let acc: Vec<_> = parts[2].strip_prefix("a=<").unwrap().strip_suffix(">").unwrap().split(',').map(|c| c.parse::<i64>().unwrap()).collect();
        Particle { id, pos: Point::from(pos), vel: Point::from(vel), acc: Point::from(acc) }
    }
}

fn main() {
    let mut particles = Vec::new();

    for (i, line) in fs::read_to_string("input.txt").unwrap().lines().enumerate() {
        particles.push(Particle::new(i, line));
    }

    for _ in 0..10_000_000 {
        for p in particles.iter_mut() {
            p.step();
        }
    }

    particles.sort_by(|a, b| a.pos.distance().cmp(&b.pos.distance()));
    for p in particles.iter().rev() {
        println!("{:?} with dist {}", p, p.pos.distance());
    }

    // println!("{particles:?}");
}
