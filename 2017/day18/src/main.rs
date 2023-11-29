#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

struct Pc {
    pc: i64,
    regs: HashMap<String, i64>,
    insns: Vec<Vec<String>>,
    last_sound: Option<i64>,
}

impl Pc {
    fn new(insns: Vec<Vec<String>>) -> Self {
        Self {
            insns,
            regs: HashMap::new(),
            pc: 0,
            last_sound: None,
        }
    }

    fn get_value(regs: & mut HashMap<String, i64>, value: &str) -> i64 {
        if let Ok(v) = value.parse::<i64>() {
            v
        } else {
            *regs.entry(value.to_owned()).or_default()
        }
    }

    fn run(& mut self) {
        while self.pc >= 0 && (self.pc as usize) < self.insns.len() {
            // println!("{:?}", self.regs);
            let insn = &self.insns[self.pc as usize];
            let opcode = insn[0].as_ref();
            match opcode {
                "set" | "add" | "mul" | "mod" => {
                    let v = Self::get_value(& mut self.regs, &insn[2]);
                    let dst = &insn[1];
                    self.regs.entry(dst.to_owned()).or_default();
                    match opcode {
                        "set" => *self.regs.get_mut(dst).unwrap() = v,
                        "add" => *self.regs.get_mut(dst).unwrap() += v,
                        "mul" => *self.regs.get_mut(dst).unwrap() *= v,
                        "mod" => *self.regs.get_mut(dst).unwrap() %= v,
                        _ => panic!()
                    }
                },
                "snd" => {
                    self.last_sound = Some(Self::get_value(& mut self.regs, &insn[1]));
                },
                "rcv" => {
                    if Self::get_value(& mut self.regs, &insn[1]) != 0 {
                        println!("recover {}", self.last_sound.unwrap());
                        break;
                    }
                },
                "jgz" => {
                    if Self::get_value(& mut self.regs,&insn[1]) > 0 {
                        self.pc += Self::get_value(& mut self.regs,&insn[2]);
                        // will be incremented after this match statement
                        self.pc -= 1;
                    }
                }
                _ => todo!("{insn:?}"),
            }
            
            self.pc += 1;
        }
    }
}



fn main() {
    let mut insns = Vec::new();

    for line in fs::read_to_string("input.txt").unwrap().lines() {
        insns.push(line.split_whitespace().map(|x| x.to_string()).collect::<Vec<String>>());
    }

    let mut pc0 = Pc::new(insns.clone());
    pc0.run();   
}
