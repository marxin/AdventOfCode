use std::cmp::Reverse;
use std::collections::BinaryHeap;
#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

const MOVES: [(i64, i64); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

#[allow(unused)]
use itertools::Itertools;

// const DEPTH: i64 = 510;
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

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
enum Wearing {
    Nothing,
    Torch,
    ClimbingGear,
}

// 0 - rocky
// 1 - wet
// 2 - narrow

fn can_use(region: i64, wear: &Wearing) -> bool {
    match region {
        0 => match wear {
            Wearing::Nothing => false,
            Wearing::Torch | Wearing::ClimbingGear => true,
        },
        1 => match wear {
            Wearing::Torch => false,
            Wearing::ClimbingGear | Wearing::Nothing => true,
        },
        2 => match wear {
            Wearing::Nothing | Wearing::Torch => true,
            Wearing::ClimbingGear => false,
        },
        _ => panic!(),
    }
}

fn main() {
    // let target = (10, 10);
    let target = (14, 796);

    let mut values: HashMap<(i64, i64), i64> = HashMap::new();

    values.insert((0, 0), calc(0));
    values.insert(target, calc(0));
    get((target.0 + 1000, target.1 + 1000), &mut values);

    let mut best: HashMap<Wearing, HashMap<(i64, i64), i64>> = HashMap::new();
    best.insert(Wearing::Nothing, HashMap::new());
    best.insert(Wearing::ClimbingGear, HashMap::new());
    best.insert(Wearing::Torch, HashMap::new());

    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, (0, 0), Wearing::Torch)));
    let mut seen = HashSet::new();

    for y in 0..=target.1 {
        for x in 0..=target.0 {
            let region = values[&(x, y)] % 3;
            match region {
                0 => print!("."),
                1 => print!("="),
                2 => print!("|"),
                _ => panic!(),
            }
        }
        println!();
    }
    println!();

    loop {
        let Reverse((time, pos, wear)) = heap.pop().unwrap();
        let region = values[&pos] % 3;
        assert!(can_use(region, &wear));
        best.get_mut(&wear).unwrap().entry(pos).or_insert(time);
        // println!("{pos:?} {time}");

        if pos == target && wear == Wearing::Torch {
            println!("DONE = {time}");
            break;
        }

        // 1) try to change what we wear
        let mut candidates = Vec::new();
        if can_use(region, &Wearing::Nothing) {
            candidates.push(Wearing::Nothing);
        }
        if can_use(region, &Wearing::Torch) {
            candidates.push(Wearing::Torch);
        }
        if can_use(region, &Wearing::ClimbingGear) {
            candidates.push(Wearing::ClimbingGear);
        }

        for c in candidates {
            if c != wear {
                let candidate = Reverse((time + 7, pos, c.clone()));
                if !best[&c].contains_key(&pos) && !seen.contains(&candidate) {
                    heap.push(candidate.clone());
                    seen.insert(candidate);
                }
            }
        }

        // 2) move to neighbor pixels
        for m in MOVES {
            let next = (pos.0 + m.0, pos.1 + m.1);
            let candidate = Reverse((time + 1, next, wear.to_owned()));
            if next.0 >= 0
                && next.1 >= 0
                && can_use(values[&next] % 3, &wear)
                && !best[&wear].contains_key(&next)
                && !seen.contains(&candidate)
            {
                heap.push(candidate.clone());
                seen.insert(candidate);
            }
        }
    }
}
