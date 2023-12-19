use std::cmp::Ordering;
#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
struct Point(i32, i32);

#[allow(dead_code)]
const MOVES: [Point; 4] = [Point(0, 1), Point(1, 0), Point(0, -1), Point(-1, 0)];

#[derive(Debug)]
enum GoToTarget {
    Accept,
    Reject,
    Workflow(String),
}

#[derive(Debug)]
enum Rule {
    GoTo(GoToTarget),
    Condition {
        key: char,
        ordering: Ordering,
        value: i32,
        goto: GoToTarget,
    },
}

fn get_goto(label: &str) -> GoToTarget {
    match label {
        "A" => GoToTarget::Accept,
        "R" => GoToTarget::Reject,
        _ => GoToTarget::Workflow(label.to_string()),
    }
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let content_parts = content.split("\n\n").collect_vec();
    let mut workflows = HashMap::new();

    for line in content_parts[0].lines() {
        // hdj{m>838:A,pv}
        let parts = line.split('{').collect_vec();
        let name = parts[0].to_string();
        let rules = parts[1]
            .strip_suffix('}')
            .unwrap()
            .split(',')
            .map(|part| {
                let parts2 = part.split(':').collect_vec();
                let chars = parts2[0].chars().collect_vec();
                if part.contains(':') {
                    let value = String::from_iter(chars.iter().skip(2))
                        .parse::<i32>()
                        .unwrap();
                    Rule::Condition {
                        key: chars[0],
                        ordering: if chars[1] == '>' {
                            Ordering::Greater
                        } else {
                            Ordering::Less
                        },
                        value,
                        goto: get_goto(parts2[1]),
                    }
                } else {
                    Rule::GoTo(get_goto(part))
                }
            })
            .collect_vec();
        workflows.insert(name, rules);
    }

    let objects: Vec<HashMap<char, i32>> = content_parts[1]
        .lines()
        .map(|line| {
            // {x=2127,m=1623,a=2188,s=1013}
            line.strip_prefix('{')
                .unwrap()
                .strip_suffix('}')
                .unwrap()
                .split(',')
                .map(|part| {
                    let mut parts = part.split('=');
                    let key = parts.next().unwrap().chars().next().unwrap();
                    let value = parts.next().unwrap().parse::<i32>().unwrap();
                    (key, value)
                })
                .collect()
        })
        .collect_vec();

    println!("{workflows:?}");
    println!("{objects:?}");

    let mut total = 0;

    for obj in objects {
        let mut workflow_name = "in".to_string();
        'outer: loop {
            let workflow = workflows.get(&workflow_name).unwrap();
            for rule in workflow {
                match rule {
                    Rule::GoTo(target) => match target {
                        GoToTarget::Accept => {
                            total += obj.values().sum::<i32>();
                            break 'outer;
                        }
                        GoToTarget::Reject => {
                            break 'outer;
                        }
                        GoToTarget::Workflow(name) => {
                            workflow_name = name.to_owned();
                            break;
                        }
                    },
                    Rule::Condition {
                        key,
                        ordering,
                        value,
                        goto,
                    } => {
                        if obj[key].cmp(value) == *ordering {
                            match goto {
                                GoToTarget::Accept => {
                                    total += obj.values().sum::<i32>();
                                    break 'outer;
                                }
                                GoToTarget::Reject => {
                                    break 'outer;
                                }
                                GoToTarget::Workflow(name) => {
                                    workflow_name = name.to_owned();
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{total}");
}
