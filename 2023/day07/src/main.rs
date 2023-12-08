#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

const VALUES: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: String,
    bid: usize,
}

impl Hand {
    fn new(cards: String, bid: usize) -> Self {
        assert_eq!(5, cards.chars().count());
        Self { cards, bid }
    }

    fn counts(&self) -> Vec<usize> {
        let jokers = self.cards.chars().filter(|&x| x == 'J').count();
        let mut freqs = self
            .cards
            .chars()
            .filter(|&c| c != 'J')
            .sorted()
            .group_by(|x| *x)
            .into_iter()
            .map(|(_, v)| v.count())
            .filter(|x| *x > 1)
            .collect_vec();
        freqs.sort();
        freqs.reverse();

        if !freqs.is_empty() {
            freqs[0] += jokers;
        } else {
            match jokers {
                0 => {}
                1..=4 => {
                    return vec![jokers + 1];
                }
                5 => {
                    return vec![jokers];
                }
                _ => panic!(),
            }
        }

        freqs
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let freqs = self.counts();
        let other_freqs = other.counts();

        if freqs != other_freqs {
            return freqs.cmp(&other_freqs);
        }

        for (c1, c2) in self.cards.chars().zip(other.cards.chars()) {
            let p1 = VALUES.iter().position(|&x| x == c1).unwrap();
            let p2 = VALUES.iter().position(|&x| x == c2).unwrap();
            if p1 != p2 {
                return p2.cmp(&p1);
            }
        }

        std::cmp::Ordering::Equal
    }
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let mut hands = Vec::new();

    for line in content.lines() {
        let parts = line.split_whitespace().collect_vec();
        let hand = Hand::new(parts[0].to_owned(), parts[1].parse().unwrap());
        hands.push(hand);
    }

    for h in &hands {
        println!("{h:?} {:?}", h.counts());
    }

    hands.sort();

    let suma: usize = hands.iter().enumerate().map(|(i, v)| (i + 1) * v.bid).sum();
    println!("{suma}");
}
