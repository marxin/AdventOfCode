#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

struct Generator {
    value: u64,
    factor: u64,
    multiple_of: u64,
}

impl  Generator {
    fn next_value(& mut self) {
        self.value = (self.value * self.factor) % 2147483647;
    }

    fn next(& mut self) {
        loop {
            self.next_value();
            if self.value % self.multiple_of == 0 {
                break;
            }
        }
    }

    fn low16(& self) -> u64 {
        self.value & ((1 << 16) - 1)
    }
}

fn main() {
    let mut a = Generator {
        value: 116,
        factor: 16807,
        multiple_of: 4,
    };
    let mut b = Generator {
        value: 299,
        factor: 48271,
        multiple_of: 8,
    };

    let mut times = 0;
    for i in 0..5_000_000 {
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
