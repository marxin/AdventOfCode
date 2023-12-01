#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[derive(Debug)]
struct Action {
    set_bit: bool,
    shift: i32,
    state: char,
}

#[derive(Debug)]
struct State {
    zero: Action,
    one: Action
}

fn main() {
    let lines: Vec<_> = fs::read_to_string("input.txt").unwrap().lines().map(|l| l.to_string()).collect();

    let steps = lines[1].split_whitespace().rev().skip(1).next().unwrap().parse::<i32>().unwrap();
    let mut it = lines.iter().skip(3);
    let mut states = HashMap::new();

    while let Some(state) = it.next() {
        let c = state.chars().rev().skip(1).next().unwrap();
        it.next();
        let line = it.next();
        let write0 = line.unwrap().chars().rev().skip(1).next().unwrap();
        let line = it.next();
        let direction0 = line.unwrap().split_whitespace().last().unwrap().strip_suffix('.').unwrap().to_string();
        let line = it.next();
        let state0 = line.unwrap().chars().rev().skip(1).next().unwrap();

        it.next();
        let line = it.next();
        let write1 = line.unwrap().chars().rev().skip(1).next().unwrap();
        let line = it.next();
        let direction1 = line.unwrap().split_whitespace().last().unwrap().strip_suffix('.').unwrap().to_string();
        let line = it.next();
        let state1 = line.unwrap().chars().rev().skip(1).next().unwrap();
        it.next();

        let state = State {
            zero: Action { set_bit: write0 == '1', shift: if direction0 == "left" { -1 } else { 1 }, state: state0 },
            one: Action { set_bit: write1 == '1', shift: if direction1 == "left" { -1 } else { 1 }, state: state1 },
        };

        states.insert(c, state);
    }

    println!("{states:?}");

    let mut pos = 0;
    let mut state = 'A';
    let mut bits: HashSet<i32> = HashSet::new();

    for _ in 0..steps {
        let current = &states[&state];
        let action = if bits.contains(&pos) {
            &current.one
        } else {
            &current.zero
        };

        if action.set_bit {
            bits.insert(pos);
        } else {
            bits.remove(&pos);
        }

        pos += action.shift;
        state = action.state;
    }

    println!("{}", bits.len());
}
