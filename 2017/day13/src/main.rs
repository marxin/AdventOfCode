#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

#[derive(Debug, Clone)]
struct Gate {
    range: i32,
    position: i32,
    direction: i32,
}

impl Gate {
    fn new(range: i32) -> Self {
        assert!(range > 1);
        Self {
            range,
            position: 0,
            direction: 1,
        }
    }

    fn step(& mut self) {
        if self.position == 0 {
            self.direction = 1;
        } else if self.position == self.range - 1 {
            self.direction = -1;
        }
        self.position += self.direction;
    }
}

fn score(mut gates: HashMap<i32, Gate>, start: i32) -> i32 {
    let mut total = 0;

    for _ in 0..start {
        for gate in gates.values_mut() {
            gate.step();
        }
    }

    let maximum = gates.keys().max().unwrap();

    for n in 0..=(*maximum) {
        if let Some(gate) = gates.get(&n) {
            if gate.position == 0 {
                total += n * gate.range;
            }
        }

        for gate in gates.values_mut(   ) {
            gate.step();
        }
    }

    total
}

fn main() {
    let mut values = vec![true; 10_000_000];

    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let parts: Vec<_> = line.split(": ").collect();
        let position = -parts[0].parse::<i32>().unwrap();
        let period = 2 * (parts[1].parse::<i32>().unwrap() - 1);

        for k in  0.. {
            let v = k * period + position;
            if v >= 0 {
                let v = v as usize;
                if v < values.len() {
                    values[v as usize] = false;
                } else {
                    break;
                }
            }
        }
    }

    println!("{:?}", values.iter().position(|&p| p));

    panic!();

    let mut gates = HashMap::new();
    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let parts: Vec<_> = line.split(": ").collect();
        gates.insert(parts[0].parse::<i32>().unwrap(), Gate::new(parts[1].parse::<i32>().unwrap()));
    }

    for i in 0.. {
        println!("{i}");
        let score = score(gates.clone(), i);
        if score == 0 {
            println!("{i}");
            break;
        }
    }
}
