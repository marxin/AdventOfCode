use std::{ops, collections::{HashMap, VecDeque}};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Pos {
    x: i32,
    y: i32
}

impl Pos {
    const KEY: i32 = 1352;

    fn is_wall(&self) -> bool {
        let v = self.x * self.x + 3 * self.x + 2 * self.x * self.y + self.y + self.y * self.y + Self::KEY;
        v.count_ones() % 2 == 1
    }
}

impl ops::Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Pos {x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

const START: Pos = Pos {x: 1, y: 1};
const MOVES: [Pos; 4] = [Pos {x: 1, y: 0}, Pos {x: 0, y: 1}, Pos {x: 0, y: -1}, Pos {x: -1, y: 0}];

fn main() {
    let start = START;
    let mut wall_cache = HashMap::new();
    let mut map = HashMap::from([(start.clone(), 0)]);
    let mut queue = VecDeque::from([start.clone()]);

    while !queue.is_empty() {
        let pos = queue.pop_front().unwrap();
        let steps = *map.get(&pos).unwrap();
        if steps < 50 {
            for next in MOVES {
                let pos2 = pos.clone() + next;
                if pos2.x < 0 || pos2.y < 0 {
                    continue;
                }

                if !map.contains_key(&pos2) {
                    let wall = *wall_cache.entry(pos2.clone()).or_insert(pos2.is_wall());
                    if !wall {
                        map.insert(pos2.clone(), steps + 1);
                        queue.push_back(pos2);
                    }
                }
            }
        }
    }

    println!("keys={}", map.len());
}
