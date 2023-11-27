#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

struct Generator {
    value: u64,
    factor: u64,
}

impl  Generator {
    fn next(& mut self) {
        self.value = (self.value * self.factor) % 2147483647;
    }

    fn low16(& self) -> u64 {
        self.value & ((1 << 16) - 1)
    }
}

fn main() {
    let mut a = Generator {
        value: 116,
        factor: 16807,
    };
    let mut b = Generator {
        value: 299,
        factor: 48271,
    };

    let mut times = 0;
    for i in 0..40_000_000 {
        a.next();
        b.next();
        if a.low16() == b.low16() {
            times += 1;
        }
        //println!("a={} b={}", a.value, b.value);
        //println!("{} {}", a.low16(), b.low16());
    }

    println!("{times}");
}
