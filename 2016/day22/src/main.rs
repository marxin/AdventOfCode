use std::{fs, collections::{HashMap, VecDeque, HashSet}};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Disk {
    x: i32,
    y: i32,
    size: usize,
    used: usize,
    available: usize,
}

impl Disk {
    fn new(line: &str) -> Self {
        let parts: Vec<_> = line.split_whitespace().collect();
        assert_eq!(parts.len(), 5);
        let coords: Vec<_> = parts[0].split('-').collect();
        assert_eq!(coords.len(), 3);
        let x = coords[1].strip_prefix('x').unwrap().parse().unwrap();
        let y = coords[2].strip_prefix('y').unwrap().parse().unwrap();
        let size = parts[1].strip_suffix('T').unwrap().parse().unwrap();
        let used = parts[2].strip_suffix('T').unwrap().parse().unwrap();
        let available = parts[3].strip_suffix('T').unwrap().parse().unwrap();
        assert!(used <= size);

        Disk { x, y, size, used, available }
    }


}

const MOVES: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
const MAX: i32 = 30;

fn main() {
    // All nodes are connected and we have all nodes in range [0-30]x[0-30].
    let mut disks = HashMap::new();

    for line in fs::read_to_string("input.txt").unwrap().lines().skip(2) {
        let disk = Disk::new(line);
        disks.insert((disk.x, disk.y), disk);
    }

    let place = disks.values().find(|p| p.used == 0).unwrap();
    assert_eq!(place.used, 0);
    let fit = place.available;
    println!("Space: {place:?}");
    println!("Fit: {fit}");

    let mut viable = 0;

    println!("Disks: {}", disks.len());
    for disk in disks.values() {
        for disk2 in disks.values() {
            if *disk != *disk2 && disk.used > 0 && disk.used < disk2.available {
                viable += 1;
            }
        }
    }

    println!("Viable: {viable}");

    for y in 0..=MAX {
        for x in 0..=MAX {
            let disk = disks.get(&(x, y)).unwrap();
            let c = {
                if x == 0 && y == 0 {
                    'S'
                }
                else if x == MAX && y == 0 {
                    'G'
                }
                else if disk.used == 0 {
                    '_'
                } else if disk.used > fit {
                    '#'
                } else {
                    '.'
                }
            };
            print!("{c}");
        }
        println!();
    }

    println!("Part 2: {}", 9+27+25+29*5+1);
}
