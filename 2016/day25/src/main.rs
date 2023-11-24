use std::{collections::HashMap, fs};

#[derive(Debug)]
struct Cpu {
    pc: i32,
    registers: HashMap::<String, i32>,
    instructions: Vec<Vec<String>>
}

impl Cpu {
    fn new(a: i32) -> Cpu {
        Cpu {pc: 0, registers: HashMap::from([(String::from("a"), a), (String::from("b"), 0), (String::from("c"), 0), (String::from("d"), 0)]), instructions: vec![]}
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

    fn run(& mut self) -> bool {
        let mut clock_expected = 0;
        let mut clock_times = 0;

        while self.pc >= 0 && self.pc < self.instructions.len() as i32 {
            let insn = &self.instructions[self.pc as usize];
            
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
                        self.pc += self.get_value(&insn[2]);
                        continue;
                    }
                }
                "tgl" => {
                    let value = self.get_value(&insn[1]);
                    let insn_index = self.pc + value;
                    if insn_index >= 0 {
                        let insn_index = insn_index as usize;
                        if insn_index < self.instructions.len() {
                            let replacement = match self.instructions[insn_index][0].as_str() {
                                "inc" => "dec",
                                "dec" => "inc",
                                "jnz" => "cpy",
                                "cpy" => "jnz",
                                "tgl" => "inc",
                                _ => panic!(),
                            };
                            self.instructions[insn_index][0] = replacement.to_string();
                            println!("change instruction {insn_index} ({value}) to {replacement}");
                        }
                    }
                },
                "out" => {
                    let value = self.get_value(&insn[1]);
                    if value != clock_expected {
                        return false;
                    }
                    clock_times += 1;
                    if clock_times == 1000 {
                        return true;
                    }
                    clock_expected = (clock_expected + 1) % 2;
                }
                _ => { panic!(); }
            }
            self.pc += 1;            
        }

        panic!()
    }
}

fn main() {
    for i in 0.. {
        println!("a={i}");
        let mut cpu = Cpu::new(i);
        for line in fs::read_to_string("input.txt").unwrap().lines() {
            cpu.instructions.push(line.split_whitespace().map(|token| String::from(token)).collect());
        }
        if cpu.run() {
            println!("DONE={i}");
            break;
        }
    }
}
