use std::ops::{Add, Mul, Sub};
#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
struct Point(i64, i64);

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul<i64> for Point {
    type Output = Point;

    fn mul(self, rhs: i64) -> Self::Output {
        Point(self.0 * rhs, self.1 * rhs)
    }
}

#[allow(dead_code)]
const MOVES: [Point; 4] = [Point(0, 1), Point(1, 0), Point(0, -1), Point(-1, 0)];
const MOVE_CHARS: [char; 4] = ['v', '>', '^', '<'];

#[allow(dead_code)]
const MOVES_WITH_DIAGONAL: [Point; 8] = [
    Point(0, 1),
    Point(1, 0),
    Point(0, -1),
    Point(-1, 0),
    Point(1, 1),
    Point(1, -1),
    Point(-1, 1),
    Point(-1, -1),
];

#[allow(dead_code)]
fn flood_fill<T: Clone, F: Fn(&Point, &T, &Point, &T) -> bool>(
    map: &HashMap<Point, T>,
    directions: &[Point],
    predicate: F,
) -> Vec<HashSet<Point>> {
    let mut groups = Vec::new();
    let mut visited = HashSet::new();

    for (point, c) in map.iter() {
        if visited.contains(point) {
            continue;
        }
        visited.insert(*point);

        let mut group = HashSet::from([*point]);
        let mut queue = VecDeque::from([*point]);

        while let Some(p) = queue.pop_front() {
            for m in directions.iter() {
                let next = p + *m;
                if let Some(v) = map.get(&next) {
                    if predicate(&p, c, &next, v) && !visited.contains(&next) {
                        queue.push_back(next);
                        group.insert(next);
                        visited.insert(next);
                    }
                }
            }
        }

        groups.push(group);
    }

    groups
}

fn print_map(map: &HashMap<Point, char>, width: i64, height: i64, pos: &Point) {
    for y in 0..height {
        for x in 0..width {
            let p = Point(x, y);
            if p == *pos {
                print!("@");
            } else {
                print!("{}", map[&p])
            }
        }
        println!();
    }
    println!();
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let (map_part, commands_part) = content.split_once("\n\n").unwrap();

    let mut map = HashMap::new();
    let mut start = None;

    let moves = commands_part
        .trim()
        .chars()
        .filter_map(|c| MOVE_CHARS.iter().position(|&x| x == c).map(|p| MOVES[p]))
        .collect_vec();

    let lines = map_part.lines().collect_vec();

    let width = lines.first().unwrap().len() as i64;
    let height = lines.len() as i64;

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let p = Point(x as i64, y as i64);
            match c {
                '@' => {
                    map.insert(p, '.');
                    start = Some(p);
                }
                c @ ('#' | '.' | 'O') => {
                    map.insert(p, c);
                }
                _ => todo!("unexpected token"),
            }
        }
    }
    let mut pos = start.unwrap();
    print_map(&map, width, height, &pos);

    for (i, &m) in moves.iter().enumerate() {
        let next = pos + m;
        if map[&next] == '.' {
            pos = next;
        } else {
            let obstacles = (1i64..)
                .map(|i| pos + m * i)
                .take_while(|p| map[p] == 'O')
                .collect_vec();
            if !obstacles.is_empty() && map[&(pos + m * ((obstacles.len() as i64) + 1))] == '.' {
                for o in obstacles.iter().rev() {
                    map.insert(*o, '.');
                }
                for o in obstacles.into_iter() {
                    map.insert(o + m, 'O');
                }
                pos = next;
            }
        }
        //println!("Moves: {}", i + 1);
        //print_map(&map, width, height, &pos);
    }

    let score = map
        .iter()
        .filter_map(|(p, v)| {
            if *v == 'O' {
                Some(p.0 + 100 * p.1)
            } else {
                None
            }
        })
        .sum::<i64>();
    dbg!(score);
}
