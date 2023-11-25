#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, fs};

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

enum Token {
    Garbage,
    Group,
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut stack = Vec::new();
    let mut depth = 0;
    let mut score = 0;
    let mut it = input.chars();

    while let Some(c) = it.next() {
        let in_garbage = stack.last().is_some_and(|f| matches!(f, Token::Garbage));

        match c {
            '!' => {
                it.next();
            },
            '<' => {
                if !in_garbage {
                    stack.push(Token::Garbage);
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
                }
            },
            '}' => {
                if !in_garbage {
                    stack.pop();
                    score += depth;
                    depth -= 1;
                }
            }
            _ => {},
        }
    }

    println!("{score}")
}
