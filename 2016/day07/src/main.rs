use std::{collections::HashSet, fs};

fn supports_ssl(packet: &str) -> bool {
    let data: Vec<char> = packet.chars().collect();
    let mut outside = HashSet::new();
    let mut inside = HashSet::new();
    let mut in_mask = false;

    for index in 0..data.len() - 2 {
        let c = data[index];
        match c {
            '[' => {
                in_mask = true;
                continue;
            }
            ']' => {
                in_mask = false;
                continue;
            }
            _ => {}
        }

        if data[index] == data[index + 2] && data[index] != data[index + 1] {
            if !in_mask {
                inside.insert(vec![data[index], data[index + 1], data[index]]);
            } else {
                outside.insert(vec![data[index + 1], data[index], data[index + 1]]);
            }
        }
    }

    inside.intersection(&outside).count() > 0
}

fn main() {
    assert_eq!(supports_ssl("aba[bab]xyz"), true);
    assert_eq!(supports_ssl("xyx[xyx]xyx"), false);
    assert_eq!(supports_ssl("aaa[kek]eke"), true);
    assert_eq!(supports_ssl("zazbz[bzb]cdb"), true);

    let mut counter = 0;
    for line in fs::read_to_string("input.txt").unwrap().lines() {
        if supports_ssl(line) {
            counter += 1;
        }
    }

    println!("{counter}");
}
