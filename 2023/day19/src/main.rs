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

#[derive(Debug, Clone)]
struct Interval {
    start: i32,
    end: i32,
}

impl Interval {
    fn split(&self, x: i32, ord: Ordering) -> (Option<Interval>, Option<Interval>) {
        match ord {
            Ordering::Less => {
                if x <= self.start {
                    (None, Some(self.clone()))
                } else if x >= self.end {
                    (Some(self.clone()), None)
                } else {
                    (
                        Some(Interval {
                            start: self.start,
                            end: x,
                        }),
                        Some(Interval {
                            start: x,
                            end: self.end,
                        }),
                    )
                }
            }
            Ordering::Greater => {
                if x >= self.end - 1 {
                    (None, Some(self.clone()))
                } else if x <= self.start - 1 {
                    (Some(self.clone()), None)
                } else {
                    (
                        Some(Interval {
                            start: x + 1,
                            end: self.end,
                        }),
                        Some(Interval {
                            start: self.start,
                            end: x + 1,
                        }),
                    )
                }
            }
            _ => panic!(),
        }
    }

    fn size(&self) -> u64 {
        (self.end - self.start) as u64
    }
}

#[derive(Debug)]
struct Object {
    vals: HashMap<char, Interval>,
    times: u64,
}

fn process(mut object: Object, start: &str, workflows: &HashMap<String, Vec<Rule>>) -> u64 {
    println!("{object:?}");
    let workflow_name = start.to_string();
    let mut total = 0;

    loop {
        let workflow = workflows.get(&workflow_name).unwrap();
        for rule in workflow {
            match rule {
                Rule::GoTo(target) => match target {
                    GoToTarget::Accept => {
                        total += object.times;
                        return total;
                    }
                    GoToTarget::Reject => return total,
                    GoToTarget::Workflow(name) => {
                        return total + process(object, name, workflows);
                    }
                },
                Rule::Condition {
                    key,
                    ordering,
                    value,
                    goto,
                } => {
                    let interval = object.vals[key].clone();
                    let (true_interval, false_interval) = interval.split(*value, *ordering);
                    if let Some(true_interval) = true_interval {
                        let subsize = object.times / interval.size() * true_interval.size();
                        let mut subobject = Object {
                            vals: object.vals.clone(),
                            times: subsize,
                        };
                        *subobject.vals.get_mut(&key).unwrap() = true_interval;
                        match goto {
                            GoToTarget::Accept => total += subobject.times,
                            GoToTarget::Reject => {}
                            GoToTarget::Workflow(name) => {
                                total += process(subobject, name, workflows);
                            }
                        }
                    }

                    if let Some(false_interval) = false_interval {
                        let subsize = object.times / interval.size() * false_interval.size();
                        *object.vals.get_mut(&key).unwrap() = false_interval;
                        object.times = subsize;
                    } else {
                        return total;
                    }
                }
            }
        }
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

    let vals = HashMap::from(['x', 'm', 'a', 's'].map(|c| {
        (
            c,
            Interval {
                start: 1,
                end: 4000 + 1,
            },
        )
    }));
    let object = Object {
        vals,
        times: 4000u64.pow(4),
    };

    println!("{}", process(object, "in", &workflows));
}
