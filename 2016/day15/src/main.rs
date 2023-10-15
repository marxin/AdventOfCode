use std::fs;

#[derive(Debug, Clone)]
struct Disc {
    pos: i32,
    num_pos: i32
}

impl Disc {
    fn add(& mut self, n: i32) {
        self.pos = (self.pos + n) % self.num_pos;
    }
}

fn combine(a: Disc, b: &Disc) -> Disc {
    let start = if a.pos != 0 { a.num_pos - a.pos } else { 0 };
    let mut x = a.pos + start;
    let mut y = b.pos + start;

    while x != y {
        if x < y {
            x += a.num_pos;
        } else {
            y += b.num_pos;
        }
    }

    Disc { pos: x, num_pos: a.num_pos * b.num_pos }
}

fn is_fine_for_all_discs(discs: &Vec<Disc>, time: i32) -> bool {
    for disc in discs {
        if (disc.pos + time) % disc.num_pos != 0 {
            return false;
        }
    }
    true
}

fn main() {
    let mut discs = Vec::new();

    for line in std::fs::read_to_string("input.txt").unwrap().lines() {
        let tokens: Vec<_> = line.split_whitespace().collect();
        let num_pos = tokens[3].parse::<i32>().unwrap();
        let pos = tokens.last().unwrap().strip_suffix('.').unwrap().parse::<i32>().unwrap();
        discs.push(Disc {pos: pos, num_pos: num_pos});
    }

    for i in 0..discs.len() {
        discs[i].add((i as i32) + 1);
    }

    println!("{:?}", discs);
    for time in 0.. {
        if is_fine_for_all_discs(&discs, time) {
            println!("first time in {}", time);
            break;
        }
    }

    let mut disc = discs[0].clone();
    for i in 1..discs.len() {
        disc = combine(disc, &discs[i]);
        println!("#{i}: {disc:?}")
    }
}
