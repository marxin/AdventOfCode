#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
struct Point(i32, i32);

#[allow(dead_code)]
const MOVES: [Point; 4] = [Point(0, 1), Point(1, 0), Point(0, -1), Point(-1, 0)];

#[allow(dead_code)]
const MOVES_WITH_DIAGONAL: [Point; 8] = [
    Point(0, 1),
    Point(1, 0),
    Point(0, -1),
    Point(-1, 0),
    Point(1, 1),
    Point(1, -1),
    Point(-1, 1),
    Point(-1, -1),
];

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let numbers = content
        .trim()
        .chars()
        .map(|c| {
            let s = c.to_string();
            s.parse::<u64>().unwrap()
        })
        .collect_vec();

    let size = numbers.iter().sum::<u64>();
    let mut vec = vec![None; size as usize];

    let mut tomove = Vec::new();

    let mut idx: u64 = 0;
    let mut index = 0;
    for x in numbers.chunks(2) {
        let full = x[0];
        tomove.push((index as usize, full as usize, idx));
        for i in 0..full {
            vec[(index + i) as usize] = Some(idx);
        }
        if let Some(skip) = x.get(1) {
            index += skip;
        }

        index += full;
        idx += 1;
    }

    tomove.reverse();

    //println!("{vec:?}");

    let mut end = vec.len() - 1;

    println!("{}", tomove.len());
    let digits: usize = tomove.iter().map(|x| x.1).sum();
    for (pos, length, idx) in tomove {
        let mut start = 0usize;

        while start < pos {
            if vec[start].is_some() {
                start += 1;
                continue;
            } else if vec.iter().skip(start).take(length).all(|x| x.is_none()) {
                vec.iter_mut()
                    .skip(start)
                    .take(length)
                    .for_each(|x| *x = Some(idx));
                vec.iter_mut()
                    .skip(pos)
                    .take(length)
                    .for_each(|x| *x = None);
                // println!("Moved to {start}");
                break;
            } else {
                start += 1;
            }
        }
    }

    let s = vec
        .iter()
        .enumerate()
        .map(|(i, v)| if let Some(v) = v { i as u64 * v } else { 0 })
        .sum::<u64>();

    let digits_after: usize = vec.iter().filter(|x| x.is_some()).count();
    assert_eq!(digits, digits_after);
    dbg!(s);
}
