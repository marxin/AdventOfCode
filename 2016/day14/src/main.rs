use std::collections::HashMap;

use md5;

const SALT: &str = "cuanljph";
const KEY_LIMIT: usize = 64;
const HASHING_REPEATS: usize = 2016 + 1;

fn get_hash(index: usize) -> String {
    let mut hash: String = format!("{SALT}{index}");

    for _ in 0..HASHING_REPEATS {
        hash = format!("{:x}", md5::compute(hash));
    }

    hash
}

fn first_triplet(i: usize, hash_map: & mut HashMap::<usize, String>) -> Option<char> {
    let hash = hash_map.entry(i).or_insert_with(|| get_hash(i));

    let chars: Vec<_> = hash.chars().collect();
    for i in 0..chars.len() - 2 {
        if chars[i] == chars[i + 1] && chars[i + 1] == chars[i + 2] {
            return Some(chars[i]);
        }
    }

    None
}

fn next_thousand(start: usize, c: char, hash_map: & mut HashMap::<usize, String>) -> bool {
    let mut needle = String::from(c).repeat(5);

    for i in start..start + 1000 {
        let hash = hash_map.entry(i).or_insert_with(|| get_hash(i));
        if hash.contains(&needle) {
            return true;
        }
    }

    false
}

fn main() {
    let mut hashes = HashMap::new();
    let mut found = 0;

    for i in 0usize.. {
        let triplet_char = first_triplet(i, & mut hashes);
        if let Some(c) = triplet_char {
            if next_thousand(i + 1, c, & mut hashes) {
                found += 1;
                println!("found key #{found} at {i}");
            }
        }

        if found == KEY_LIMIT {
            break;
        }
    }    
}
