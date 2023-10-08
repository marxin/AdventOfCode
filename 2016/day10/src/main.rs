use std::{fs, collections::HashMap, process::exit};

#[derive(Debug)]
struct Robot {
    id: usize,
    done:  bool,
    lower: Option<usize>,
    higher: Option<usize>,
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
                match parts[5] {
                    "output" => {}
                    "bot" => {
                        lowid = Some(parts[6].parse::<usize>().unwrap());
                    }
                    _ => { panic!();
                    }
                }
                match parts[10] {
                    "output" => {}
                    "bot" => {
                        highid = Some(parts[11].parse::<usize>().unwrap());
                    }
                    _ => {
                        panic!()
                    }
                }

                let robot = Robot { id, done: false, lower: lowid, higher: highid, values: vec![] };
                robots.insert(robot.id, robot);
            }
            _ => todo!()
        }
    };

    for (index, value) in inputs {
        let r = robots.get_mut(&index).unwrap();
        r.values.push(value);
    }

     while robots.iter().any(|r| !r.1.done) {
        // println!("{:?}", robots);
        for i in 0..robots.len() {
            let robot = robots.get_mut(&i).unwrap();
            if !robot.done && robot.values.len() == 2 {
                robot.done = true;
                robot.values.sort();

                if robot.values == vec![17, 61] {
                    print!("DONE={:?}", robot);
                    exit(0);
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

    
}
