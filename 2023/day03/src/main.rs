#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn are_neighbors(multiply: &(i32, i32), number: &((i32, i32), i32)) -> bool {
    let number_length = number.1.to_string().len();
    for x in number.0.0..number.0.0 + (number_length as i32) {
        let dx = (multiply.0 - x).abs();
        let dy = (multiply.1 - number.0.1).abs();
        if dx <= 1 && dy <= 1 {
            return true;
        }
    }

    false
}

fn main() {   
    let content = fs::read_to_string("input.txt").unwrap();
    let mut map = HashMap::new();
    let mut reachable = HashSet::new();

    for (y, line) in content.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                map.insert((x as i32, y as i32), c);
                if !c.is_numeric() {
                    reachable.insert((x as i32, y as i32));
                }
            }
        }
    }

    // propagate reachable locations
    let mut queue = VecDeque::from_iter(reachable.iter().copied());
    while let Some(entry) = queue.pop_back() {
        for x in -1..=1 {
            for y in -1..=1 {
                if x != 0 || y != 0 {
                    let pos = (entry.0 + x, entry.1 + y);
                    if map.contains_key(&pos) && !reachable.contains(&pos) {
                        reachable.insert(pos);
                        queue.push_back(pos);
                    }
                }
            }
        }
    }

    let mut numbers = Vec::new();

    // part 2
    for (y, line) in content.lines().enumerate() {
        let mut chars: Vec<_> = line.chars().enumerate().collect();
        let mut x = 0;
        while x < chars.len() {
            if chars[x].1.is_digit(10) {
                let value = String::from_iter(chars[x..].iter().take_while(|c| c.1.is_digit(10)).map(|c| c.1));
                let number: i32 = value.parse().unwrap();
                numbers.push(((x as i32, y as i32), number));
                x += value.len() + 1;
            } else {
                x += 1;
            }
        }
    }

    let mut multiply_map = HashMap::new();
    for (pos, c) in map.iter() {
        if *c == '*' {
            multiply_map.insert(*pos, Vec::new());
        }
    }

    for (key, value) in multiply_map.iter_mut() {
        for number in numbers.iter() {
            if are_neighbors(key, number) {
                value.push(number.1);
            }
        }
    }

    let mut total = 0;
    println!("{multiply_map:?}");

    for value in multiply_map.values() {
        if value.len() == 2 {
            total += value[0] * value[1];
        }
    }

    println!("{total}");
}
