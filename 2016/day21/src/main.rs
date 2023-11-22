use std::{fs, collections::VecDeque, ops::Index};

#[derive(Debug)]
enum Operation {
    SwapPosition {
        source: usize,
        target: usize,
    },
    SwapLetter {
        source_letter: char,
        target_letter: char,
    },
    RotateLeft {
        n: usize,
    },
    RotateRight {
        n: usize,
    },
    RotateOnPosition {
        letter: char,
    },
    ReverseRange {
        start: usize,
        end: usize,
    },
    MovePosition {
        index: usize,
        insert: usize,
    },
}

impl Operation {
    fn make_step(&self, vec: VecDeque<char>) -> VecDeque<char> {
        match self {
            Self::SwapPosition { source, target } => {
                let mut vec = vec.clone();
                vec.swap(*source, *target);
                vec
            },
            Self::SwapLetter { source_letter, target_letter } => {
                vec.iter().map(|c| {
                    if c == source_letter {
                        *target_letter
                    } else if c == target_letter {
                        *source_letter
                    } else {
                        *c
                    }                    
                }).collect()
            },
            Self::ReverseRange { start, end } => {
                let mut result = VecDeque::new();
                result.extend(vec.range(..start));
                result.extend(vec.range(start..=end).rev());
                result.extend(vec.range(end + 1..));
                result
            },
            Self::RotateLeft { n } => {
                let mut vec = vec.clone();
                vec.rotate_left(*n);
                vec
            },
            Self::RotateRight { n } => {
                let mut vec = vec.clone();
                vec.rotate_right(*n);
                vec
            },
            Self::MovePosition { index, insert } => {
                let mut vec = vec.clone();
                let c = vec.remove(*index).unwrap();
                vec.insert(*insert, c);
                vec
            },
            Self::RotateOnPosition { letter } => {
                let mut vec = vec.clone();
                let mut position = vec.iter().position(|c| c == letter).unwrap();
                if position >= 4 {
                    position += 1;
                }
                position += 1;
                vec.rotate_right(position % vec.len());
                vec
            }
        }
    }
}

fn parse_operations() -> Vec<Operation> {
    let mut operations = Vec::new();

    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let parts: Vec<_> = line.split_whitespace().collect();
        let op = match (parts[0], parts[1]) {
            ("swap", "position") =>
                Operation::SwapPosition { source: parts[2].parse().unwrap(), target: parts.last().unwrap().parse().unwrap() },
            ("swap", "letter") => Operation::SwapLetter { source_letter: parts[2].chars().next().unwrap(), target_letter: parts.last().unwrap().chars().next().unwrap() },
            ("rotate", "left") => Operation::RotateLeft { n: parts[2].parse().unwrap() },
            ("rotate", "right") => Operation::RotateRight { n: parts[2].parse().unwrap() },
            ("rotate", "based") => Operation::RotateOnPosition { letter: parts.last().unwrap().chars().next().unwrap() },
            ("reverse", "positions") => Operation::ReverseRange { start: parts[2].parse().unwrap(), end: parts[4].parse().unwrap() },
            ("move", "position") => Operation::MovePosition { index: parts[2].parse().unwrap(), insert: parts.last().unwrap().parse().unwrap() },
            _ => panic!("unsupported operation: {line}")
        };
        operations.push(op);
    }

    operations
}

fn main() {
    let operations = parse_operations();

    let mut pwd: VecDeque<_> = "abcde".chars().collect();
    for op in operations {
        println!("{op:?}");
        pwd = op.make_step(pwd);
        println!("{}", String::from_iter(pwd.iter()));
    }
}
