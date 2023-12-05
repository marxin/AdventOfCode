#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};
use itertools::Itertools;
use bytesize::ByteSize;

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Mapping {
    range: Range,
    dest: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn new(start: usize, len: usize) -> Self {
        Range { start, end: start + len }
    }

    fn len(&self) -> usize {
        self.end - self.start
    }

    fn contains(&self, n: usize) -> bool {
        self.start <= n && n < self.end
    }

    fn split_by_range(&self, other: &Vec<Range>) -> Vec<Range> {
        let mut values = vec![self.start];

        for r in other {
            if self.contains(r.start) {
                values.push(r.start);
            }
            if self.contains(r.end) {
                values.push(r.end);
            }
        }

        if !(values.last().unwrap() == &self.end) {
            values.push(self.end);
        }

        values.windows(2).map(|pair| Range::new(pair[0], pair[1] - pair[0])).collect_vec()
    }

    fn map_entire_range(&self, mapping: &Vec<Mapping>) -> Self {
        for m in mapping {
            if m.range.start <= self.start && self.end <= m.range.end {
                let offset = self.start - m.range.start;
                return Range::new(m.dest + offset, self.len());
            }
        }

        self.clone()
    }
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let mut it = content.lines().into_iter();
    let mut conversion = HashMap::new();

    let seeds: Vec<usize> = it.next().unwrap().split(':').last().unwrap().split_whitespace().map(|token| token.parse().unwrap()).collect();
    it.next();
    while let Some(line) = it.next() {
        let parts: Vec<_> = line.split_whitespace().next().unwrap().split('-').collect();
        let mut mapping = Vec::new();

        loop {
            let line = it.next().unwrap();
            if line == "" {
                println!("{mapping:?}");
                break;
            } else {
                let numbers: Vec<_> = line.split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();
                mapping.push(Mapping { range: Range::new(numbers[1], numbers[2]), dest: numbers[0]});
            }
        }

        mapping.sort();
        conversion.insert(parts[0].to_string(), (parts[2].to_string(), mapping));
    }

    test_ranges();

    let mut key = "seed".to_string();
    let mut ranges = seeds.chunks(2).map(|x| Range::new(x[0], x[1])).sorted().collect_vec();
    println!("init ranges: {ranges:?}");

    while let Some((next_key, mapping)) = conversion.get(&key) {
        let mut next_ranges = Vec::new();
        let mapping_ranges = mapping.iter().map(|x| x.range.clone()).collect_vec();
        for r in ranges {
            for subr in r.split_by_range(&mapping_ranges) {
                next_ranges.push(subr.map_entire_range(mapping));
            }
        }
        next_ranges.sort();
        println!("after {next_key}: {next_ranges:?}");
        ranges = next_ranges;
        key = next_key.clone();
    }
}

fn test_ranges() {
    let r = Range::new(0, 3);
    assert_eq!(3, r.len());

    // seeds: 79 14 55 13
    // seed-to-soil map:
    // 50 98 2
    // 52 50 48

    let r = Range::new(79, 14);
    let ranges = vec![Range::new(50, 48), Range::new(98, 2)];
    assert_eq!(r.split_by_range(&ranges), vec![r.clone()]);

    let r = Range::new(50, 20);
    let ranges = vec![Range::new(10, 50), Range::new(63, 100)];
    let result = r.split_by_range(&ranges);
    assert_eq!(result, vec![Range::new(50, 10), Range::new(60, 3), Range::new(63, 7)]);

    let r = Range::new(50, 20);
    let ranges = vec![Range::new(10, 50), Range::new(63, 100)];
    let result = r.split_by_range(&ranges);
    assert_eq!(result, vec![Range::new(50, 10), Range::new(60, 3), Range::new(63, 7)]);

    let r = Range::new(50, 20);
    let mapping = vec![Mapping { range: Range::new(10, 100), dest: 1000}];
    let splitted = r.map_entire_range(&mapping);
    assert_eq!(splitted, Range::new(1040, 20));
}
