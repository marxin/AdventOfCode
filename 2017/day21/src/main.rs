#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

type Morph = HashMap<(usize, usize), char>;

fn parse(line: &str) -> Morph {
    let mut map = HashMap::new();
    for (y, part) in line.split('/').enumerate() {
        for (x, c) in part.chars().enumerate() {
            map.insert((x, y), c);
        }
    }

    map
}

fn get_size(source: &Morph) -> usize {
    let size = f32::sqrt(source.len() as f32);
    size.round() as usize
}

fn rotate(source: &Morph) -> Morph {
    let size = get_size(source);

    let mut m = Morph::new();
    for (k, v) in source {
        m.insert((size - k.1 - 1, k.0), *v);
    }

    m
}

fn flip_horizontaly(source: &Morph) -> Morph {
    let size = get_size(source);

    let mut m = Morph::new();
    for (k, v) in source {
        m.insert((size - k.0 - 1, k.1), *v);
    }

    m
}

fn all(source: &Morph) -> Vec<Morph> {
    let mut x = source.clone();
    let mut v = Vec::new();
    for _ in 0..4 {
        v.push(x.clone());
        x = rotate(&x);
    }
    x = flip_horizontaly(&x);
    for _ in 0..4 {
        v.push(x.clone());
        x = rotate(&x);
    }

    v
}

fn step(pos: Morph, mapping: &Vec<(Morph, Morph)>) -> Morph {
    let mut result = HashMap::new();
    let size = get_size(&pos);
    let part_size = if size % 2 == 0 {
        2
    } else {
        3
    };
    let multiplier = size / part_size;

    for x in 0..multiplier {
        for y in 0..multiplier {
            // cut the part from the entire Morph
            let x0 = x * part_size;
            let y0 = y * part_size;

            let mut part = HashMap::new();
            for xx in x * part_size..(x + 1) * part_size {
                for yy in y * part_size..(y + 1) * part_size {
                    part.insert((xx - x0, yy - y0), *pos.get(&(xx, yy)).unwrap());
                }
            }

            // find the transformation
            let mut next = None;
            for m in mapping {
                if part == m.0 {
                    next = Some(m.1.clone());
                    //println!("using pattern:");
                    //print(&m.0);
                    //println!("->");
                    //print(&m.1);
                    break;
                }
            }

            let next = next.unwrap();
            // shift it back

            for (k, v) in next {
                result.insert((k.0 + (x * (part_size + 1)), k.1 + y * (part_size + 1)), v);
            }
        }
    }

    result
}

fn print(pos: &Morph) {
    let size = get_size(&pos);

    for y in 0..size {
        for x in 0..size {
            print!("{}", pos.get(&(x, y)).unwrap());
        }
        println!()
    }
    println!();
}

fn main() {
    let mut mapping = Vec::new();

    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let parts: Vec<_> = line.split(" => ").collect();
        assert_eq!(2, parts.len());
        mapping.push((parse(parts[0]), parse(parts[1])));
    }

    let mut mapping_full = Vec::new();

    for m in mapping {
        for c in all(&m.0) {
            mapping_full.push((c, m.1.clone()));
        }
    }

    let mut x = parse("#./..");
    let y = x.clone();
    for _ in 0..4 {
        x = rotate(&x);
    }
    assert_eq!(x, y);

    for _ in 0..2 {
        x = flip_horizontaly(&x);
    }
    assert_eq!(x, y);

    println!("MAPPING size = {}", mapping_full.len());

    let mut start = parse(".#./..#/###");
    print(&start);
    for i in 0..18 {
        println!("step #{i} with size = {}", get_size(&start));
        start = step( start, &mapping_full);
        // print(&start);
    }

    let on = start.values().filter(|&&x| x == '#').count();
    println!("{on}");
}
