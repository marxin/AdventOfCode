#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn cypher(N: usize, rotations: &Vec<usize>) -> VecDeque<usize> {
    let mut chain = VecDeque::from_iter(0..N);
    let mut total_rot = 0;

    for n in 0..64 {
        for (i, &rotation) in rotations.iter().enumerate() {
            let rest = chain.split_off(rotation);
            chain = chain.into_iter().rev().collect();
            chain.extend(rest);
            let r = (i + (n * rotations.len()) + rotation) % chain.len();
            chain.rotate_left(r);
            total_rot += r;
        }
    }

    chain.rotate_right(total_rot % chain.len());
    chain
}

fn calculate_hash(s: &str) -> String {
    let mut rotations: Vec<_> = s.chars().map(|c| c as usize).collect();
    rotations.extend(vec![17, 31, 73, 47, 23]);
    let chain: Vec<usize> = cypher(256, &rotations).into();

    let mut result = String::from("");
    for chunk in chain.chunks_exact(16) {
        let digit = chunk.iter().fold(0, |acc, x| acc ^ x);
        result.push_str(&format!("{:08b}", digit));
    }

    result
}

fn main() {
    let key = "wenycdww";
    let mut count = 0;

    for i in 0..128 {
        let h = calculate_hash(format!("{key}-{i}").as_ref());
        let bits = h.chars().filter(|&c| c == '1').count();
        assert_eq!(128, h.len());
        count += bits;
    }

    println!("{count}");
}
