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
    let mut map = HashMap::new();

    for y in 0..128 {
        let h = calculate_hash(format!("{key}-{y}").as_ref());
        let bits = h.chars().filter(|&c| c == '1').count();
        for (x, b) in h.chars().enumerate() {
            if b == '1' {
                map.insert((x as i32, y), 0);
            }
        }

        assert_eq!(128, h.len());
        count += bits;
    }

    println!("{count}");
    assert_eq!(count, map.len());

    let mut done = 0;
    let mut group_id = 1;

    while done != count {
        let start = map.iter().filter_map(|tuple| if tuple.1 == &0 { Some(*tuple.0) } else { None}).next().unwrap();
        *map.get_mut(&start).unwrap() = 1;

        let mut queue = VecDeque::from(vec![start]);
        let mut seen = HashSet::new();
        seen.insert(start);
        done += 1;

        while let Some(p) = queue.pop_back() {

            for m in MOVES {
                let pos2 = (p.0 + m.0, p.1 + m.1);
                if let Some(v) = map.get_mut(&pos2) {
                    if !seen.contains(&pos2) {
                        *v = group_id;
                        queue.push_back(pos2);
                        seen.insert(pos2);
                        done += 1;
                    }
                }
            }
        }

        group_id += 1;
    }

    println!("{}", group_id - 1);
}
