use std::{fs, cmp};
use std::cmp::Ordering;

#[derive(PartialEq, PartialOrd, Eq, Debug)]
struct Interval {
    start: u32,
    end: u32
}

impl Ord for Interval {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let r = self.start.cmp(&other.start);
        if r == Ordering::Equal {
            self.end.cmp(&other.end)
        } else {
            r
        }
    }
}

impl Interval {
    fn contains(&self, value: u32) -> bool {
        self.start <= value && value <= self.end
    }
}

fn main() {
    let mut intervals = Vec::new();
    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let parts: Vec<_> = line.split('-').collect();
        intervals.push(Interval { start: parts[0].parse::<u32>().unwrap(), end: parts[1].parse::<u32>().unwrap() });
    }

    intervals.sort();

    let mut ip = 0;
    for i in 0..intervals.len() {
        let interval = &intervals[i];
        if interval.contains(ip) {
            ip = cmp::max(ip, interval.end + 1);
        } else if ip < interval.start {
            println!("found IP = {ip}");
            break;
        }

    }
}
