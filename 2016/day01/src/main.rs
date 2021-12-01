use std::{collections, process::exit};

use collections::HashSet;

fn distance(pos: &(i32, i32)) -> i32 {
    pos.0.abs() + pos.1.abs()
}

fn record(pos: (i32, i32), seen: &mut HashSet<(i32, i32)>) {
    if seen.contains(&pos) {
        println!("Found it with distance: {}", distance(&pos));
        exit(0);
    }

    seen.insert(pos);
}

fn main() {
    let input = String::from("R4, R5, L5, L5, L3, R2, R1, R1, L5, R5, R2, L1, L3, L4, R3, L1, L1, R2, R3, R3, R1, L3, L5, R3, R1, L1, R1, R2, L1, L4, L5, R4, R2, L192, R5, L2, R53, R1, L5, R73, R5, L5, R186, L3, L2, R1, R3, L3, L3, R1, L4, L2, R3, L5, R4, R3, R1, L1, R5, R2, R1, R1, R1, R3, R2, L1, R5, R1, L5, R2, L2, L4, R3, L1, R4, L5, R4, R3, L5, L3, R4, R2, L5, L5, R2, R3, R5, R4, R2, R1, L1, L5, L2, L3, L4, L5, L4, L5, L1, R3, R4, R5, R3, L5, L4, L3, L1, L4, R2, R5, R5, R4, L2, L4, R3, R1, L2, R5, L5, R1, R1, L1, L5, L5, L2, L1, R5, R2, L4, L1, R4, R3, L3, R1, R5, L1, L4, R2, L3, R5, R3, R1, L3");
    let parts = input.split(", ");
    let mut seen:HashSet<(i32, i32)> = HashSet::new();

    let mut pos = (0, 0);
    let mut orientation = 0;
    let moves = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    println!("=== start ===");

    for item in parts {
        let direction = item.chars().nth(0);
        orientation += match direction {
            Some('L') => 3,
            Some('R') => 1,
            _ => 0
        };
        orientation %= 4;

        let n:i32 = (&item[1..]).parse().unwrap();

        for _ in 1..n + 1 {
            pos.0 += moves[orientation].0;
            pos.1 += moves[orientation].1;

            record(pos, &mut seen);
        }
    }

    println!("{}", distance(&pos));
}
