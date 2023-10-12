use std::{collections::HashMap, fs};

#[derive(Debug)]
struct Cpu {
    pc: i32,
    registers: HashMap::<String, i32>,
    instructions: Vec<Vec<String>>
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {pc: 0, registers: HashMap::from([(String::from("a"), 0), (String::from("b"), 0), (String::from("c"), 0), (String::from("d"), 0)]), instructions: vec![]}
    }

    fn get_value(&self, item: &String) -> i32 {
        match self.registers.get(item) {
            Some(x) => {
                *x
            }
            None => {
                item.parse::<i32>().unwrap()
            }
        }
    }

    fn run(& mut self) {
        while self.pc >= 0 && self.pc < self.instructions.len() as i32 {
            let insn = &self.instructions[self.pc as usize];
            if self.pc == 9 || self.pc > 15 {
                println!("pc: {}, insn: {insn:?} {:?}", self.pc, self.registers);
            }
            // println!("pc: {}, insn: {insn:?} {:?}", self.pc, self.registers);
            match insn[0].as_str() {
                "cpy" => {
                    let value = self.get_value(&insn[1]);
                    *self.registers.get_mut(&insn[2]).unwrap() = value;
                }
                "inc" => {
                    *self.registers.get_mut(&insn[1]).unwrap() += 1;
                }
                "dec" => {
                    *self.registers.get_mut(&insn[1]).unwrap() -= 1;
                }
                "jnz" => {
                    let value = self.get_value(&insn[1]);
                    if value != 0 {
                        self.pc += insn[2].parse::<i32>().unwrap();
                        continue;
                    }
                }
                _ => { panic!(); }
            }
            self.pc += 1;            
        }
    }
}

fn main() {
    let mut cpu = Cpu::new();
    for line in fs::read_to_string("input.txt").unwrap().lines() {
        cpu.instructions.push(line.split_whitespace().map(|token| String::from(token)).collect());
    }

    println!("{cpu:?}");
    cpu.run();
    println!("a={}", cpu.registers.get(&String::from("a")).unwrap());
}
