use std::{cmp::Reverse, hash::Hash};
#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
struct Point(i32, i32);

#[allow(dead_code)]
const MOVES: [Point; 4] = [Point(0, 1), Point(1, 0), Point(0, -1), Point(-1, 0)];

#[allow(dead_code)]
const MOVES_WITH_DIAGONAL: [Point; 8] = [
    Point(0, 1),
    Point(1, 0),
    Point(0, -1),
    Point(-1, 0),
    Point(1, 1),
    Point(1, -1),
    Point(-1, 1),
    Point(-1, -1),
];

fn is_sorted(rules: &Vec<(i64, i64)>, numbers: &[i64]) -> bool {
    let map: HashMap<i64, usize> =
        HashMap::from_iter(numbers.iter().enumerate().map(|(i, v)| (*v, i)));
    rules.iter().all(|(lhs, rhs)| {
        let l = map.get(lhs);
        let r = map.get(rhs);

        if let (Some(l), Some(r)) = (l, r) {
            l < r
        } else {
            true
        }
    })
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let (rules, lines) = content.split_once("\n\n").unwrap();

    let lines = lines.lines().collect_vec();
    let rules = rules
        .lines()
        .map(|l| {
            let parts = l.split_once("|").unwrap();
            (
                parts.0.parse::<i64>().unwrap(),
                parts.1.parse::<i64>().unwrap(),
            )
        })
        .collect_vec();

    let mut sum = 0;
    for line in lines {
        let numbers = line
            .split(",")
            .map(|x| x.parse::<i64>().unwrap())
            .collect_vec();
        if !is_sorted(&rules, &numbers) {
            // println!("{numbers:?}");
            let valid_rules = rules
                .iter()
                .filter(|(x, y)| numbers.contains(x) && numbers.contains(y))
                .collect_vec();

            let mut freq = HashMap::new();
            for (lhs, _rhs) in valid_rules {
                freq.entry(lhs).and_modify(|v| *v += 1).or_insert(1);
            }

            assert!(freq.values().all_unique());
            let sorted = freq
                .into_iter()
                .sorted_by_key(|x| Reverse(x.1))
                .map(|x| x.0)
                .collect_vec();

            sum += sorted[sorted.len() / 2];
        }
    }

    dbg!(sum);
}
