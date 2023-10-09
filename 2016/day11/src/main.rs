use std::{fs, collections::HashMap, hash::Hash, hash::Hasher, ops::RangeFull};

const FLOORS: usize = 4;
const TYPES: usize = 2;

#[derive(Debug, Hash, PartialEq, Eq)]
struct Map {
    steps: usize,
    elevator: usize,
    chips: Vec<usize>,
    generators: Vec<usize>
}

impl Map {
    fn new() -> Map {
        Map {steps: 0, elevator: 0, chips: vec![usize::MAX; TYPES], generators: vec![usize::MAX; TYPES] }
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

                if self.chips[n] == floor {
                    print!("{}M ", short_namming[n]);
                } else {
                    print!(".  ");
                }
            }
            println!();
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
                    map.chips[*mineral_mapping.get(mineral).unwrap()] = i;
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

    println!("{mineral_mapping:?}");
    println!("{:?}", short_namming);
    println!("{:?}", map);
    map.print(&short_namming);
}
