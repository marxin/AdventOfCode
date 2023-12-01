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
            regs: HashMap::new(),
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

    fn run(&mut self) -> u64 {
        let mut multiplied = 0;
        while self.pc >= 0 && (self.pc as usize) < self.insns.len() {
            let insn = &self.insns[self.pc as usize];
            println!("{:?} {insn:?}", self.regs);
            let opcode = insn[0].as_ref();
            match opcode {
                "set" | "sub" | "mul" => {
                    let v = Self::get_value(&mut self.regs, &insn[2]);
                    let dst = &insn[1];
                    self.regs.entry(dst.to_owned()).or_default();
                    match opcode {
                        "set" => *self.regs.get_mut(dst).unwrap() = v,
                        "sub" => *self.regs.get_mut(dst).unwrap() -= v,
                        "mul" => {
                            *self.regs.get_mut(dst).unwrap() *= v;
                            multiplied += 1;
                        }
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
        multiplied
    }
}

fn main() {
    let mut insns = Vec::new();

    for line in fs::read_to_string("input.txt").unwrap().lines() {
        insns.push(
            line.split_whitespace()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        );
    }

    let multiplied = Pc::new(insns).run();
    println!("{multiplied}");
}
