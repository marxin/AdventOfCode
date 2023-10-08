use std::{fs, collections::HashMap, process::exit};

#[derive(Debug)]
struct Robot {
    id: usize,
    done:  bool,
    lower: Option<usize>,
    higher: Option<usize>,
    lower_output: Option<usize>,
    higher_output: Option<usize>,
    values: Vec<usize>
}

fn main() {
    let mut robots: HashMap<usize, Robot> = HashMap::new();
    let mut inputs: Vec<(usize, usize)> = Vec::new();

    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let parts: Vec<_> = line.split_whitespace().collect();
        match parts[0] {
            "value" => {
                assert_eq!(parts[4], "bot");
                inputs.push((parts[5].parse().unwrap(), parts[1].parse().unwrap()));
            }
            "bot" => {
                let id: usize = parts[1].parse().unwrap();
                let mut lowid = None;
                let mut highid = None;
                let mut lower_output = None;
                let mut higher_output = None;
                match parts[5] {
                    "output" => {
                        lower_output = Some(parts[6].parse::<usize>().unwrap());
                    }
                    "bot" => {
                        lowid = Some(parts[6].parse::<usize>().unwrap());
                    }
                    _ => { panic!();
                    }
                }
                match parts[10] {
                    "output" => {
                        higher_output = Some(parts[11].parse::<usize>().unwrap());
                    }
                    "bot" => {
                        highid = Some(parts[11].parse::<usize>().unwrap());
                    }
                    _ => {
                        panic!()
                    }
                }

                let robot = Robot { id, done: false, lower: lowid, higher: highid, lower_output, higher_output, values: vec![] };
                robots.insert(robot.id, robot);
            }
            _ => todo!()
        }
    };

    for (index, value) in inputs {
        let r = robots.get_mut(&index).unwrap();
        r.values.push(value);
    }

    let mut vals = vec![];

     while robots.iter().any(|r| !r.1.done) {
        // println!("{:?}", robots);
        for i in 0..robots.len() {
            let robot = robots.get_mut(&i).unwrap();
            if !robot.done && robot.values.len() == 2 {
                robot.done = true;
                robot.values.sort();

                if let Some(lower) = robot.lower_output {
                    if lower < 3 {
                        vals.push(robot.values[0]);
                    }
                }
                if let Some(higher) = robot.higher_output {
                    if higher < 3 {
                        vals.push(robot.values[1]);
                    }
                }

                let transform = [(robot.lower, robot.values[0]), (robot.higher, robot.values[1])];

                for i in 0..2 {
                    if let Some(index) = transform[i].0 {
                        robots.get_mut(&index).unwrap().values.push(transform[i].1);
                    }
                }
            }
        }
     }

     assert_eq!(vals.len(), 3);
     println!("{:?}", vals);
     println!("{}", vals.iter().fold(1, |acc, x| acc * x));
}
