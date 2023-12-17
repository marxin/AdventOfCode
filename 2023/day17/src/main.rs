#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
struct Point(i32, i32);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct State {
    pos: Point,
    orient: Point,
    straight: i32,
}

#[allow(dead_code)]
const MOVES: [Point; 4] = [Point(0, 1), Point(1, 0), Point(0, -1), Point(-1, 0)];

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let lines = content.lines().collect_vec();
    let mut map = HashMap::new();

    let width = lines.first().unwrap().len() as i32;
    let height = lines.len() as i32;

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert(Point(x as i32, y as i32), c.to_string().parse::<i32>().unwrap());
        }
    }

    let end = Point(width - 1, height - 1);

    let mut states: HashMap<State, i32> = HashMap::new();
    let mut queue = VecDeque::from(vec![
        (0, State { pos: Point(0, 0), orient: Point(1, 0), straight: 0 }),
        (0, State { pos: Point(0, 0), orient: Point(0, 1), straight: 0 }),
    ]);

    let mut best = i32::MAX;

    while let Some((steps, state)) = queue.pop_back() {
        //  println!("{state:?}");
        assert!(state.straight <= 3);
        if steps > best {
            continue;
        }

        if state.pos == end {
            if steps < best {
                best = steps;
                println!("New best solution: {best}, worklist size: {}", queue.len());
            }
            
        }

        if let Some(value) = states.get(&state) {
            if *value <= steps {
                // we have a better solution
                continue;
            }
        }

        states.insert(state.clone(), steps);
        
        let index = MOVES.iter().position(|&x| x == state.orient).unwrap();
        let mut orients = vec![MOVES[(index + 1) % MOVES.len()], MOVES[(index - 1) % MOVES.len()]];

        if state.straight < 3 {
            orients.push(state.orient.clone());
        }

        for orient in orients {
            let straight = if orient == state.orient {
                state.straight + 1
            } else {
                1
            };

            let next = State { orient: orient, pos: Point(state.pos.0 + orient.0, state.pos.1 + orient.1), straight: straight };
            if map.contains_key(&next.pos) {
                queue.push_back((steps + map[&next.pos], next));
            }
        }
    }
    
}
