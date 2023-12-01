#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[derive(Debug)]
struct Pc {
    pc: i64,
    regs: HashMap<String, i64>,
    insns: Vec<Vec<String>>,
}

impl Pc {
    fn new(insns: Vec<Vec<String>>) -> Self {
        Self {
            insns,
            regs: HashMap::from([("a".to_string(), 1)]),
            pc: 0,
        }
    }

    fn get_value(regs: &mut HashMap<String, i64>, value: &str) -> i64 {
        if let Ok(v) = value.parse::<i64>() {
            v
        } else {
            *regs.entry(value.to_owned()).or_default()
        }
    }

    fn run(&mut self) -> i64 {
        let mut prev: Option<HashMap<String, i64>> = None;
        while self.pc >= 0 && (self.pc as usize) < self.insns.len() {
            let insn = &self.insns[self.pc as usize];

            if self.pc == 23 {
                let add = 106489;
                let g = self.regs.get_mut(&"g".to_string()).unwrap();
                if *g == -add - 1 {
                    *g += add;
                    let d = self.regs.get_mut(&"d".to_string()).unwrap();
                    *d += add;
                }

                // println!("{} {:?} {insn:?}", self.pc, self.regs);
            }

            if self.pc == 28 {                
                print!("{} {:?} {insn:?} // ", self.pc, self.regs);
                let f = self.regs.get(&"f".to_string()).unwrap();
                let g = self.regs.get(&"g".to_string()).unwrap();
                if *f == 1 && *g == -16881 {
                    let P = 6;
                    let STEP = 17;

                    for _ in 0..165 {
                        *self.regs.get_mut(&"d".to_string()).unwrap() += STEP * P;
                        *self.regs.get_mut(&"e".to_string()).unwrap() += STEP * P;
                        *self.regs.get_mut(&"g".to_string()).unwrap() += STEP * P;
                        *self.regs.get_mut(&"b".to_string()).unwrap() += STEP * P;
                        *self.regs.get_mut(&"h".to_string()).unwrap() += P - 1;
                    }

                    *self.regs.get_mut(&"h".to_string()).unwrap() += 3;
                    *self.regs.get_mut(&"g".to_string()).unwrap() += STEP * 3;

                    println!("{} {:?} {insn:?}", self.pc, self.regs);
                    //panic!();
                }
                if let Some(p) = prev {
                    for (k, v) in self.regs.iter() {
                        print!("{k}={} ", v - p.get(k).unwrap());
                    }
                    println!();
                }
                prev = Some(self.regs.clone());
            }
            let opcode = insn[0].as_ref();
            match opcode {
                "set" | "sub" | "mul" => {
                    let v = Self::get_value(&mut self.regs, &insn[2]);
                    let dst = &insn[1];
                    self.regs.entry(dst.to_owned()).or_default();
                    match opcode {
                        "set" => *self.regs.get_mut(dst).unwrap() = v,
                        "sub" => *self.regs.get_mut(dst).unwrap() -= v,
                        "mul" => *self.regs.get_mut(dst).unwrap() *= v,
                        _ => panic!(),
                    }
                },
                "jnz" => {
                    if Self::get_value(&mut self.regs, &insn[1]) != 0 {
                        self.pc += Self::get_value(&mut self.regs, &insn[2]);
                        // will be incremented after this match statement
                        self.pc -= 1;
                    }
                }
                _ => todo!("{insn:?}"),
            }

            self.pc += 1;
        }
        self.regs[&"h".to_string()]
    }
}

use primes::is_prime;

fn main() {
    // it counts non-primes in a range, sigh :()
    let mut total = 1;
    for i in (106500..123500).step_by(17) {
        if !is_prime(i) {
            total += 1;
        }
    }

    println!("{total}");

    let mut insns = Vec::new();

    for line in fs::read_to_string("input.txt").unwrap().lines() {
        insns.push(
            line.split_whitespace()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        );
    }

    let h = Pc::new(insns).run();
    println!("{h}");
}
