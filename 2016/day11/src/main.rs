use std::{fs, collections::HashMap, hash::Hash, hash::Hasher, ops::RangeFull};

const FLOORS: usize = 4;
const TYPES: usize = 2;
const CHANGES: [i32; 2] = [-1, 1];

#[derive(Debug, Clone, Copy)]
enum Candidate {
    Microchip(usize),
    Generator(usize)
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Map {
    steps: usize,
    elevator: usize,
    microchips: Vec<usize>,
    generators: Vec<usize>
}

impl Map {
    fn new() -> Map {
        Map {steps: 0, elevator: 0, microchips: vec![usize::MAX; TYPES], generators: vec![usize::MAX; TYPES] }
    }

    fn print(&self, short_namming: &Vec<char>) {
        println!("=== steps: {} ===", self.steps);
        for floor in (0..FLOORS).rev() {
            print!("F{} ", floor + 1);
            if self.elevator == floor {
                print!("E ");
            } else {
                print!(". ");
            }
            for n in 0..TYPES {
                if self.generators[n] == floor {
                    print!("{}G ", short_namming[n]);
                } else {
                    print!(".  ");
                }

                if self.microchips[n] == floor {
                    print!("{}M ", short_namming[n]);
                } else {
                    print!(".  ");
                }
            }
            println!();
        }
    }

    fn get_candidates(&self) -> Vec<Candidate> {
        let mut candidates = vec![];
        for i in 0..TYPES {
            if self.microchips[i] == self.elevator {
                candidates.push(Candidate::Microchip(i));
            }
            if self.generators[i] == self.elevator {
                candidates.push(Candidate::Generator(i));
            }
        }

        candidates
    }

    fn get_candidate_subsets(&self) -> Vec<Vec<Candidate>> {
        let mut subsets = Vec::new();
        let candidates = self.get_candidates();

        for (i, c) in candidates.iter().enumerate() {
            subsets.push(vec![*c]);
            for j in i + 1..candidates.len() {
                subsets.push(vec![*c, candidates[j]]);
            }
        }

        subsets
    }

    fn try_all_possibilites(&self, short_namming: &Vec<char>) {
        for candidate in self.get_candidate_subsets() {
            for change in CHANGES {
                let newfloor = (self.elevator as i32) + change;
                if newfloor >= 0 && newfloor < FLOORS as i32 {
                    let mut next = self.clone();
                    next.elevator = newfloor as usize;
                    next.steps += 1;
                    for c in candidate.clone() {
                        match c {
                            Candidate::Generator(i) => {
                                next.generators[i] = next.elevator;
                            }
                            Candidate::Microchip(i) => {
                                next.microchips[i] = next.elevator;
                            }
                        }
                    }
                    println!();
                    next.print(short_namming);
                }
            }            
        }
    }
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let mut mineral_mapping = HashMap::new();
    let mut short_namming = vec![];
    let mut map = Map::new();

    for (i, line) in content.lines().enumerate() {
        let mut tokens: Vec<_> = line.strip_suffix(".").unwrap().split_whitespace().skip(4).collect();
        
        while !tokens.is_empty() {
            if tokens[0] == "nothing" {
                break;
            }
            match tokens[2] {
                "microchip" => {
                    let mut mineral = tokens[1];
                    mineral = mineral.split('-').next().unwrap();
                    if !mineral_mapping.contains_key(mineral) {
                        mineral_mapping.insert(mineral, mineral_mapping.len());
                        short_namming.push(mineral.to_uppercase().chars().next().unwrap());
                    }
                    map.microchips[*mineral_mapping.get(mineral).unwrap()] = i;
                    tokens = tokens.into_iter().skip(4).collect();
                }
                "generator" => {
                    let mineral = tokens[1];
                    if !mineral_mapping.contains_key(mineral) {
                        mineral_mapping.insert(mineral, mineral_mapping.len());
                        short_namming.push(mineral.to_uppercase().chars().next().unwrap());
                    }
                    map.generators[*mineral_mapping.get(mineral).unwrap()] = i;
                    tokens = tokens.into_iter().skip(4).collect();
                }
                _ => {
                    panic!("unknown token");
                }
            }
        }        
    }

    println!("mapping = {:?}", short_namming);
    map.print(&short_namming);
    println!("");
    map.try_all_possibilites(&short_namming);
}
