#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

#[derive(Debug)]
struct Mapping {
    dest: usize,
    src: usize,
    n: usize,
}

impl Mapping {
    fn map(&self, n: usize) -> Option<usize> {
        if self.src <= n && n < (self.src + self.n) {
            let d = n - self.src;
            Some(self.dest + d)
        } else {
            None
        }
    }
}

fn map_seed(mut seed: usize, conversion: &HashMap<String, (String, Vec<Mapping>)>) -> usize {
    let mut key = "seed".to_string();

    while key != "location" {
        let value = &conversion[&key];
        key = value.0.clone();

        for m in &value.1 {
            if let Some(d) = m.map(seed) {
                seed = d;
                break;
            }
        }
    }

    seed
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let mut it = content.lines().into_iter();
    let mut conversion = HashMap::new();

    let seeds: Vec<usize> = it.next().unwrap().split(':').last().unwrap().split_whitespace().map(|token| token.parse().unwrap()).collect();
    it.next();
    while let Some(line) = it.next() {
        let parts: Vec<_> = line.split_whitespace().next().unwrap().split('-').collect();
        let mut mapping = Vec::new();

        loop {
            let line = it.next().unwrap();
            if line == "" {
                println!("{mapping:?}");
                break;
            } else {
                let numbers: Vec<_> = line.split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();
                mapping.push(Mapping {dest: numbers[0], src: numbers[1], n: numbers[2]});
            }
        }

        conversion.insert(parts[0].to_string(), (parts[2].to_string(), mapping));
    }

    println!("{conversion:?}");
    let mapped: Vec<_> = seeds.iter().map(|x| map_seed(*x, &conversion)).collect();
    println!("{mapped:?}");
    println!("{}", mapped.iter().min().unwrap());
}
