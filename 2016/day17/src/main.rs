use std::collections::VecDeque;

use md5;

const LIMIT: i32 = 4;
const MOVES: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];
const NAMES: [char; 4] = ['U', 'D', 'L', 'R'];

fn is_open(c: char) -> bool {
    c.is_alphabetic() && c != 'a'
}

fn get_moves(code: &String) -> Vec<(char, (i32, i32))> {
    let hash = md5::compute(code);
    let hash = format!("{:x}", hash);

    let mut result = Vec::new();
    for (i, m) in MOVES.iter().enumerate() {
        if is_open(hash.chars().nth(i).unwrap()) {
            result.push((NAMES[i], m.clone()));
        }
    }

    result
}

fn main() {
    let mut worklist = VecDeque::new();
    let mykey = String::from("awrkjxxr");
    worklist.push_back((0, mykey.clone(), (0, 0)));    

    while !worklist.is_empty() {
        let (steps, key, pos) = worklist.pop_front().unwrap();
        if pos == (LIMIT - 1, LIMIT - 1) {
            let result = String::from_iter(key.chars().skip(mykey.len()));
            println!("Done in {steps}");
            continue;
        }

        for (c, m) in get_moves(&key) {
            let next = (pos.0 + m.0, pos.1 + m.1);
            if next.0 >= 0 && next.0 < LIMIT && next.1 >= 0 && next.1 < LIMIT {
                let mut next_key = key.clone();
                next_key.push(c);
                worklist.push_back((steps + 1, next_key, next));
            }
        }
    }

    
}
