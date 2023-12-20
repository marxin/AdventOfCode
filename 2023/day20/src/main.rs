#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;

#[derive(Debug)]
struct Signal {
    source: String,
    destination: String,
    high: bool,
}

#[derive(Debug)]
enum Module {
    Broadcaster,
    Conjunction { memory: HashMap<String, bool> },
    FlipFlop { enabled: bool },
}

#[derive(Debug)]
struct ModuleInfo {
    module: Module,
    dests: Vec<String>,
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let lines = content.lines().collect_vec();
    let mut modules = HashMap::new();

    let mut sources = HashMap::new();

    for (_y, line) in lines.iter().enumerate() {
        let parts = line.split(" -> ").collect_vec();

        let module = match parts.first().unwrap().chars().next().unwrap() {
            'b' => Module::Broadcaster,
            '%' => Module::FlipFlop { enabled: false },
            '&' => Module::Conjunction {
                memory: HashMap::new(),
            },
            _ => panic!(),
        };

        let name = parts[0].replace(['%', '&'], "");
        let dests = parts[1].split(", ").map(|p| p.to_string()).collect_vec();
        for d in dests.iter() {
            sources
                .entry(d.clone())
                .and_modify(|v: &mut Vec<String>| v.push(name.clone()))
                .or_insert(vec![name.clone()]);
        }

        modules.insert(name, ModuleInfo { module, dests });
    }

    for m in modules.iter_mut() {
        if matches!(&m.1.module, Module::Conjunction { .. }) {
            m.1.module = Module::Conjunction {
                memory: HashMap::from_iter(
                    sources[m.0].iter().unique().map(|x| (x.clone(), false)),
                ),
            };
        }
    }

    println!("{modules:?}");

    let mut queue = VecDeque::new();

    let mut times = HashMap::new();

    let mut cache = HashMap::new();

    for i in 0.. {
        // println!("=== #{i} ===");
        queue.push_back(Signal {
            source: "button".to_string(),
            destination: "broadcaster".to_string(),
            high: false,
        });

        while let Some(Signal {
            source,
            destination,
            high,
        }) = queue.pop_front()
        {
            if destination == "xn" && high {
                if !cache.contains_key(&source) {
                    cache.insert(source.clone(), i);
                }
                println!("#{i} {source} -> {destination} = {high} (diff: {})", i - cache[&source]);
                *cache.get_mut(&source).unwrap() = i;
            }

            times.entry(high).and_modify(|x| *x += 1).or_insert(1);
            // println!("{source} -> {destination} = {high}");

            if let Some(module) = modules.get_mut(&destination) {
                let mut tosend = None;
                match module.module {
                    Module::Broadcaster => {
                        tosend = Some(high);
                    }
                    Module::FlipFlop { ref mut enabled } => {
                        if !high {
                            *enabled = !*enabled;
                            tosend = Some(*enabled);
                        }
                    }
                    Module::Conjunction { ref mut memory } => {
                        *memory.get_mut(&source).unwrap() = high;
                        tosend = Some(!memory.values().all(|&x| x));
                    }
                }

                if let Some(pulse) = tosend {
                    for dest in module.dests.iter() {
                        queue.push_back(Signal {
                            source: destination.clone(),
                            destination: dest.clone(),
                            high: pulse,
                        });
                    }
                }
            } else {
                if destination == "rx" && !high {
                    todo!("done {high}");
                }
            }
        }

        /*
        for m in modules.iter() {
            if m.0 != "gp" {
                continue;
            }
            print!("{}: ", m.0);
            match &m.1.module {
                Module::Broadcaster => {
                    println!("broadcaster");
                },
                Module::Conjunction { memory } => {
                    println!("{:?}", memory.values());
                },
                Module::FlipFlop { enabled } => {
                    println!("{enabled}");
                }
            }
        }
        */
    }

    println!("{times:?}");
    println!("{}", times.values().product::<i32>());
}
