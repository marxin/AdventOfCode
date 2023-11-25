#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

fn cypher(N: usize, rotations: &Vec<usize>) -> VecDeque<usize> {
    println!("len={}", rotations.len());
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
    println!("{chain:?} {total_rot}");

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
        result.push_str(&format!("{:02x}", digit));
    }

    //assert_eq!(32, result.len());
    result
}

fn main() {    
    assert_eq!("a2582a3a0e66e6e86e3812dcb672a272", calculate_hash(""));
    assert_eq!("33efeb34ea91902bb2f59c9920caa6cd", calculate_hash("AoC 2017"));
    assert_eq!("3efbe78a8d82f29979031a4aa0b16a9d", calculate_hash("1,2,3"));
    assert_eq!("63960835bcdc130f0b66d7ff4f6a5a8e", calculate_hash("1,2,4"));
    assert_eq!("7adfd64c2a03a4968cf708d1b7fd418d", calculate_hash("106,16,254,226,55,2,1,166,177,247,93,0,255,228,60,36"));

    let input = fs::read_to_string("input.txt").unwrap().trim().to_string();
    println!("{}", calculate_hash(&input));
}
