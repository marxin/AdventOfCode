#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[derive(Debug)]
struct Pc {
    pc: i64,
    id: i64,
    regs: HashMap<String, i64>,
    insns: Vec<Vec<String>>,
    last_sound: Option<i64>,
    input: VecDeque<i64>,
    yielded: bool,
    sent_messages: usize,
}

impl Pc {
    fn new(insns: Vec<Vec<String>>, p: i64) -> Self {
        Self {
            insns,
            id: p,
            regs: HashMap::from([("p".to_string(), p)]),
            pc: 0,
            last_sound: None,
            input: VecDeque::new(),
            yielded: false,
            sent_messages: 0,
        }
    }

    fn get_value(regs: &mut HashMap<String, i64>, value: &str) -> i64 {
        if let Ok(v) = value.parse::<i64>() {
            v
        } else {
            *regs.entry(value.to_owned()).or_default()
        }
    }

    fn run(&mut self, other: &mut Pc) {
        self.yielded = false;
        while self.pc >= 0 && (self.pc as usize) < self.insns.len() {
            let insn = &self.insns[self.pc as usize];
            /*
            println!(
                "PC #{}: {:?} {insn:?} ({:?})",
                self.id, self.regs, self.input
            );
            */
            let opcode = insn[0].as_ref();
            match opcode {
                "set" | "add" | "mul" | "mod" => {
                    let v = Self::get_value(&mut self.regs, &insn[2]);
                    let dst = &insn[1];
                    self.regs.entry(dst.to_owned()).or_default();
                    match opcode {
                        "set" => *self.regs.get_mut(dst).unwrap() = v,
                        "add" => *self.regs.get_mut(dst).unwrap() += v,
                        "mul" => *self.regs.get_mut(dst).unwrap() *= v,
                        "mod" => *self.regs.get_mut(dst).unwrap() %= v,
                        _ => panic!(),
                    }
                }
                "snd" => {
                    let value = Self::get_value(&mut self.regs, &insn[1]);
                    other.input.push_back(value);
                    self.sent_messages += 1;
                }
                "rcv" => {
                    if let Some(value) = self.input.pop_front() {
                        let dst = &insn[1];
                        self.regs.entry(dst.to_owned()).or_default();
                        *self.regs.get_mut(dst).unwrap() = value;
                    } else {
                        // yield for now
                        println!("yielding {}", self.id);
                        self.yielded = true;
                        return;
                    }
                }
                "jgz" => {
                    if Self::get_value(&mut self.regs, &insn[1]) > 0 {
                        self.pc += Self::get_value(&mut self.regs, &insn[2]);
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
        insns.push(
            line.split_whitespace()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        );
    }

    let mut pcs = vec![Pc::new(insns.clone(), 0), Pc::new(insns.clone(), 1)];
    let mut first = true;
    loop {
        let mut iter = pcs.iter_mut();
        let pc0 = iter.next().unwrap();
        let pc1 = iter.next().unwrap();

        if (first) {
            pc0.run(pc1);
        } else {
            pc1.run(pc0);
        }

        if pc0.yielded && pc1.yielded && pc0.input.is_empty() && pc1.input.is_empty() {
            println!("done");
            break;
        }

        first = !first;
    }

    println!("{:?}", pcs[0]);
    println!("{:?}", pcs[1]);
}
