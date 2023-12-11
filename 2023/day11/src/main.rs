#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;

const TIMES: usize = 1000000 - 1;

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let width = content.lines().next().unwrap().len();
    let height = content.lines().count();

    let mut positions = Vec::new();

    for (y, line) in content.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                positions.push((x, y));
            }
        }
    }

    let empty_rows = (0..height).filter(|y| !positions.iter().any(|g| g.1 == *y)).collect_vec();
    let empty_columns = (0..width).filter(|x| !positions.iter().any(|g| g.0 == *x)).collect_vec();
    println!("empty_rows={empty_rows:?}, empty_columns={empty_columns:?}");

    positions = positions.into_iter().map(|g| {
        let xholes = empty_columns.iter().filter(|&&c| c < g.0).count();
        let yholes = empty_rows.iter().filter(|&&r| r < g.1).count();
        (g.0 + xholes * TIMES, g.1 + yholes * TIMES)
    }).collect();

    let total = positions
        .iter()
        .cartesian_product(&positions)
        .map(|(a, b)| (a.0 as i64 - b.0 as i64).abs() + (a.1 as i64 - b.1 as i64).abs())
        .sum::<i64>();

    println!("{}", total / 2);
}
