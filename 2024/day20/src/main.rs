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
fn fastest_path(
    map: &HashSet<Point>,
    start: &Point,
    end: &Point,
    limit: usize,
) -> Option<(usize, Vec<Point>)> {
    let mut todo = VecDeque::from([(*start, 0)]);
    let mut visited = HashMap::from([(*start, 0)]);

    while let Some((pos, steps)) = todo.pop_front() {
        for m in MOVES.iter() {
            let next = pos + *m;
            if !visited.contains_key(&next) && map.contains(&next) {
                visited.insert(next, steps + 1);
                if &next == end {
                    break;
                }
                todo.push_back((next, steps + 1));
            }
        }
    }

    let mut pos = *end;
    let mut steps = visited[end];
    if steps >= limit {
        return None;
    }
    let mut path = Vec::new();

    while &pos != start {
        let previous = MOVES
            .iter()
            .map(|m| pos + *m)
            .filter(|p| visited.get(p) == Some(&(steps - 1)))
            .exactly_one()
            .unwrap();
        path.push(previous);
        steps -= 1;
        pos = previous;
    }
    path.reverse();

    Some((visited[end], path))
}

fn print_map(
    maze: &HashSet<Point>,
    width: i64,
    height: i64,
    start: &Point,
    end: &Point,
    path: &Vec<Point>,
    hack: &(Point, Point),
) {
    for y in 0..height {
        for x in 0..width {
            let p = Point(x, y);
            if &p == start {
                print!("S");
            } else if &p == end {
                print!("E");
            } else if p == hack.0 {
                print!("1")
            } else if p == hack.1 {
                print!("2")
            } else if path.contains(&p) {
                print!("█");
            } else if maze.contains(&p) {
                print!(".")
            } else {
                print!("#")
            }
        }
        println!();
    }
    println!();
    println!();
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let lines = content.lines().collect_vec();

    let width = lines.first().unwrap().len() as i64;
    let height = lines.len() as i64;

    let mut maze = HashSet::new();
    let mut walls = HashSet::new();
    let mut start = None;
    let mut end = None;

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = Point(x as i64, y as i64);
            match c {
                'S' => {
                    start = Some(pos);
                    maze.insert(pos);
                }
                'E' => {
                    end = Some(pos);
                    maze.insert(pos);
                }
                '.' => {
                    maze.insert(pos);
                }
                '#' => {
                    walls.insert(pos);
                }
                _ => todo!("unexpected token"),
            }
        }
    }

    let start = start.unwrap();
    let end = end.unwrap();

    let default_path = fastest_path(&maze, &start, &end, usize::MAX).unwrap().0;
    dbg!(default_path);

    let candidates = walls
        .iter()
        .flat_map(|p| MOVES.iter().map(|m| (*p, *p + *m)))
        .filter(|p| maze.contains(&p.1))
        .collect_vec();

    dbg!(candidates.len());
    let mut histogram: HashMap<usize, usize> = HashMap::new();
    let mut total = 0;

    for (i, hack) in candidates.iter().enumerate() {
        let mut cloned_maze = maze.clone();
        cloned_maze.insert(hack.0);
        if let Some((steps, path)) = fastest_path(&cloned_maze, &start, &end, default_path) {
            if let Some(idx) = path.iter().position(|x| x == &hack.0) {
                if path.get(idx + 1) == Some(&hack.1) {
                    let diff = default_path - steps;
                    *histogram.entry(diff).or_default() += 1;
                    if diff >= 100 {
                        total += 1;
                    }
                    //if diff == 18 {
                    //print_map(&maze, width, height, &start, &end, &path, &hack);
                    //}
                }
            }
        }
    }

    //for (k, v) in histogram.iter().sorted_by_key(|x| x.0) {
    //println!("{k}:{v}");
    //}

    dbg!(total);
}
