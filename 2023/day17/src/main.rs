use std::cmp::Reverse;
#[allow(unused)]
use std::{
    collections::BinaryHeap, collections::HashMap, collections::HashSet, collections::VecDeque, fs,
};

#[allow(unused)]
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
struct Point(i32, i32);

#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
struct State {
    pos: Point,
    orient: Point,
    straight: i32,
}

#[allow(dead_code)]
const MOVES: [Point; 4] = [Point(0, 1), Point(1, 0), Point(0, -1), Point(-1, 0)];

const MIN_STEPS: i32 = 4;
const MAX_STEPS: i32 = 10;

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let lines = content.lines().collect_vec();
    let mut map = HashMap::new();

    let width = lines.first().unwrap().len() as i32;
    let height = lines.len() as i32;

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert(
                Point(x as i32, y as i32),
                c.to_string().parse::<i32>().unwrap(),
            );
        }
    }

    let start = Point(0, 0);
    let end = Point(width - 1, height - 1);
    let dist = |pos: &Point| end.0 - pos.0 + end.1 - pos.1;

    let mut states: HashMap<State, i32> = HashMap::new();
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((
        dist(&start) + 0,
        0,
        State {
            pos: start.clone(),
            orient: Point(1, 0),
            straight: 0,
        },
    )));
    heap.push(Reverse((
        dist(&start) + 0,
        0,
        State {
            pos: start.clone(),
            orient: Point(0, 1),
            straight: 0,
        },
    )));

    let mut best = i32::MAX;

    while let Some(Reverse((_, steps, state))) = heap.pop() {
        //  println!("{state:?}");
        assert!(state.straight <= MAX_STEPS);
        if steps > best {
            continue;
        }

        if state.pos == end && state.straight >= MIN_STEPS {
            if steps < best {
                best = steps;
                println!("New best solution: {best}, worklist size: {}", heap.len());
            }
        }

        if let Some(value) = states.get(&state) {
            if *value <= steps {
                // we have a better solution
                continue;
            }
        }

        states.insert(state.clone(), steps);

        let index = MOVES.iter().position(|&x| x == state.orient).unwrap();
        let mut orients = Vec::new();
        if state.straight < MAX_STEPS {
            orients.push(state.orient.clone());
        }
        if state.straight >= MIN_STEPS {
            orients.push(MOVES[(index + 1) % MOVES.len()]);
            orients.push(MOVES[(index - 1) % MOVES.len()]);
        }

        for orient in orients {
            let straight = if orient == state.orient {
                state.straight + 1
            } else {
                1
            };

            let next = State {
                orient: orient,
                pos: Point(state.pos.0 + orient.0, state.pos.1 + orient.1),
                straight: straight,
            };
            if map.contains_key(&next.pos) {
                let next_steps = steps + map[&next.pos];
                heap.push(Reverse((dist(&next.pos) + next_steps, next_steps, next)));
            }
        }
    }
}
