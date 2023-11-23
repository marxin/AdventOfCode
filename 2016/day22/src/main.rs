use std::{fs, collections::{HashMap, VecDeque, HashSet}};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Disk {
    x: usize,
    y: usize,
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
        let x: usize = coords[1].strip_prefix('x').unwrap().parse().unwrap();
        let y: usize = coords[2].strip_prefix('y').unwrap().parse().unwrap();
        let size: usize = parts[1].strip_suffix('T').unwrap().parse().unwrap();
        let used: usize = parts[2].strip_suffix('T').unwrap().parse().unwrap();
        let available: usize = parts[3].strip_suffix('T').unwrap().parse().unwrap();
        assert!(used <= size);

        Disk { x, y, size, used, available }
    }


}

const MOVES: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn main() {
    let mut disks = HashMap::new();

    for line in fs::read_to_string("input.txt").unwrap().lines().skip(2) {
        let disk = Disk::new(line);
        disks.insert((disk.x, disk.y), disk);
    }

    let mut viable = 0;

    println!("Disks: {}", disks.len());
    for disk in disks.values() {
        for disk2 in disks.values() {
            if *disk != *disk2 && disk.used > 0 && disk.used < disk2.available {
                viable += 1;
            }
        }
    }

    println!("{viable}");
}
