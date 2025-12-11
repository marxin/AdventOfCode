#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;

fn find_best(joltage: Vec<i64>, buttons: &Vec<Vec<i64>>, mut pressed: usize) -> Option<usize> {
    if joltage.iter().all(|x| *x == 0) {
        return Some(pressed);
    } else if joltage.iter().any(|x| *x < 0) {
        return None;
    }

    let min_value = *joltage.iter().filter(|&&x| x > 0).min().unwrap();
    let minimal_key = joltage.iter().position(|&x| x == min_value).unwrap() as i64;

    let buttons_with_min = buttons
        .iter()
        .filter(|&keys| keys.contains(&minimal_key))
        .cloned()
        .collect_vec();
    let next_buttons = buttons
        .iter()
        .filter(|&kb| !buttons_with_min.contains(kb))
        .cloned()
        .collect_vec();

    pressed += min_value as usize;

    let mut best: Option<usize> = None;
    let combs = combination_k(min_value as _, buttons_with_min.len());
    // println!(
    //     "doing {min_value} at index {minimal_key} of {} combs with {} keyboards, next buttons: {}",
    //     combs.len(),
    //     buttons_with_min.len(),
    //     next_buttons.len()
    // );
    for combination in combs {
        let mut joltage = joltage.clone();
        for (button, times) in buttons_with_min.iter().zip(combination) {
            for b in button {
                joltage[*b as usize] -= times;
            }
        }

        if let Some(b) = find_best(joltage, &next_buttons, pressed) {
            best = Some(best.unwrap_or_default().max(b));
        }
    }

    best
}

fn combinations(values: Vec<i64>, n: usize, budget: usize, result: &mut Vec<Vec<i64>>) {
    if n == 0 {
        if budget == 0 {
            result.push(values);
        }
        return;
    }

    for i in 0..=budget {
        let mut values2 = values.clone();
        values2.push(i as _);
        combinations(values2, n - 1, budget - i, result);
    }
}

fn combination_k(budget: usize, n: usize) -> Vec<Vec<i64>> {
    let mut results = Vec::new();
    combinations(Vec::new(), n, budget, &mut results);
    results
}

fn main() {
    dbg!(combination_k(50, 5).len());

    let content = fs::read_to_string("input.txt").unwrap();
    let lines = content.lines().collect_vec();

    for (lineno, line) in lines.iter().enumerate() {
        let parts = line.split_whitespace().collect_vec();
        println!("{}: {}", lineno + 1, parts[0]);

        let buttons = parts
            .iter()
            .skip(1)
            .rev()
            .skip(1)
            .rev()
            .map(|part| {
                part.strip_prefix("(")
                    .unwrap()
                    .strip_suffix(")")
                    .unwrap()
                    .split(",")
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect_vec()
            })
            .sorted_by_key(|x| x.len())
            .rev()
            .collect_vec();

        let joltage = parts
            .last()
            .unwrap()
            .strip_prefix("{")
            .unwrap()
            .strip_suffix("}")
            .unwrap()
            .split(",")
            .map(|n| n.parse::<i64>().unwrap())
            .collect_vec();

        println!("best={}", find_best(joltage, &buttons, 0).unwrap());
    }
}
