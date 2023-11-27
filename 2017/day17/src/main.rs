#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

fn main() {
    let mut next_values: HashMap<usize, usize> = HashMap::new();
    next_values.insert(0, 0);
    let mut pos: usize = 0;
    let step = 301;

    for i in 1..=50000000 {
        for _ in 0..step {
            pos = *next_values.get(&pos).unwrap();
        }

        next_values.insert(i, *next_values.get(&pos).unwrap());
        *next_values.get_mut(&pos).unwrap() = i;
        pos = i;

        println!("{i}");
    }

    println!("{}", next_values.get(&0).unwrap());
}
