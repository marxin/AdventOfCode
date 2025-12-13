#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;

fn calculate_combinations(joltage: usize, keyboards: usize) -> usize {
    if keyboards <= 1 {
        return keyboards;
    }

    let top = (keyboards + joltage - 1) as u128;
    let bottom = (keyboards - 1) as u128;

    ((1..=top).rev().take(bottom as usize).product::<u128>() / (1..=bottom).product::<u128>())
        as usize
}

#[test]
fn test_calculate_combinations() {
    assert_eq!(calculate_combinations(10, 4), 84);
}

fn dump_keyboard_frequency(buttons: &[Vec<i64>]) {
    let fl = buttons.iter().flatten().sorted().collect_vec();
    for i in 0..10i64 {
        println!("{i}: {}x", fl.iter().filter(|x| ***x == i).count());
    }
}

fn find_best(
    depth: usize,
    joltage: Vec<i64>,
    buttons: &[Vec<i64>],
    mut pressed: usize,
) -> Option<usize> {
    if joltage.iter().all(|x| *x == 0) {
        return Some(pressed);
    } else if joltage.iter().any(|x| *x < 0) || buttons.is_empty() {
        return None;
    }

    let joltage_pairs = joltage.iter().enumerate().to_owned().collect_vec();
    let (combinations, buttons_with_min, min_value, min_index) = joltage_pairs
        .iter()
        .filter(|x| *x.1 > 0)
        .map(|(i, v)| {
            let buttons_with_min = buttons
                .iter()
                .filter(|&keys| keys.contains(&(*i as i64)))
                .cloned()
                .collect_vec();
            (
                calculate_combinations(**v as usize, buttons_with_min.len()),
                buttons_with_min,
                **v,
                *i,
            )
        })
        .min()
        .unwrap();

    let next_buttons = buttons
        .iter()
        .filter(|&kb| !buttons_with_min.contains(kb))
        .cloned()
        .collect_vec();

    if depth == 0 {
        dbg!((depth, combinations, min_value, min_index, &joltage));
        dump_keyboard_frequency(&next_buttons);
    }

    pressed += min_value as usize;

    let mut best: Option<usize> = None;
    let combs = combination_k(min_value as _, buttons_with_min.len());
    assert_eq!(combs.len(), combinations);

    let mut counter = 0;
    for combination in combs {
        counter += 1;
        let mut joltage = joltage.clone();
        for (button, times) in buttons_with_min.iter().zip(combination) {
            for b in button {
                joltage[*b as usize] -= times;
            }
        }

        if let Some(b) = find_best(depth + 1, joltage, &next_buttons, pressed) {
            best = Some(best.unwrap_or_default().max(b));
            dbg!(best);
        }

        if depth == 0 && (combinations / 400) % counter == 0 {
            let progress = 100.0 * counter as f32 / combinations as f32;
            println!("{progress:.0}% ({counter}/{combinations})");
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
    // dbg!((budget, n));
    let mut results = Vec::new();
    combinations(Vec::new(), n, budget, &mut results);
    results
}

fn main() {
    // dbg!(combination_k(5, 2));
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

        println!("best={}", find_best(0, joltage, &buttons, 0).unwrap());
    }
}
