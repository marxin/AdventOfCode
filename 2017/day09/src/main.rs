#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, fs};

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

enum Token {
    Garbage,
    Group,
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap().trim().to_string();
    let mut stack = Vec::new();
    let mut depth = 0;
    let mut score = 0;
    let mut it = input.chars();
    let mut garbage_size = 0;

    while let Some(c) = it.next() {
        let in_garbage = stack.last().is_some_and(|f| matches!(f, Token::Garbage));

        match c {
            '!' => {
                it.next();
            },
            '<' => {
                if !in_garbage {
                    stack.push(Token::Garbage);
                } else {
                    garbage_size += 1;
                }
            },
            '>' => {
                assert!(in_garbage);
                stack.pop();
            },
            '{' => {
                if !in_garbage {
                    stack.push(Token::Group);
                    depth += 1;
                } else {
                    garbage_size += 1;
                }
            },
            '}' => {
                if !in_garbage {
                    stack.pop();
                    score += depth;
                    depth -= 1;
                } else {
                    garbage_size += 1;
                }
            }
            _ => { 
                if in_garbage {
                garbage_size += 1;
                }
            },
        }
    }

    println!("{score}");
    println!("{garbage_size}");
}
