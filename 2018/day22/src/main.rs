#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;

const DEPTH: i64 = 5355;

fn calc(value: i64) -> i64 {
    (value + DEPTH) % 20183
}

fn get(pos: (i64, i64), values: &mut HashMap<(i64, i64), i64>) -> i64 {
    if let Some(v) = values.get(&pos) {
        *v
    } else {
        let mut value = if pos.0 == 0 {
            48271 * pos.1
        } else if pos.1 == 0 {
            16807 * pos.0
        } else {
            get((pos.0 - 1, pos.1), values) * get((pos.0, pos.1 - 1), values)
        };
        value = calc(value);
        values.insert(pos.clone(), value);
        value
    }
}

fn main() {
    let target = (14, 796);

    let mut values: HashMap<(i64, i64), i64> = HashMap::new();

    get(target, &mut values);
    values.insert((0, 0), calc(0));
    values.insert(target, calc(0));

    assert_eq!(values.len() as i64, (target.0 + 1) * (target.1 + 1));
    let total = values.values().map(|v| v % 3).sum::<i64>();

    println!("{:?}", total);
}
