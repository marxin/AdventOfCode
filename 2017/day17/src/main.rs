#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

fn main() {
    let mut buffer = vec![0];
    let mut pos = 0;
    let step = 301;

    for i in 1..=2017{
        pos += step;
        pos = pos % buffer.len();
        buffer.insert(pos + 1, i);
        pos += 1;
    }

    println!("{}", buffer[pos + 1]);
}
