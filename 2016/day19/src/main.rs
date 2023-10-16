use std::collections::VecDeque;

fn eliminate_slow(n: usize) -> usize {
    let mut elves:VecDeque<_> = (1..=n).collect();

    while elves.len() > 2 {
        let mut i = 0;
        let mut c = 0;
        let chunk = elves.len() / 3;
        while i < chunk {
            c += 1;
            if c == 1000 {
                println!("len={}", elves.len());
                c = 0;
            }

            let n = i + elves.len() / 2;
            elves.remove(n);
            i += 1;
        }
        elves.rotate_left(chunk);
    }

    match elves.len() {
        1 | 2 => { *elves.front().unwrap() }
        3 => { *elves.back().unwrap() }
        _ => { panic!("wrong number of elements"); }
    }
}

fn eliminate_slow2(n: usize) -> usize {
    let mut elves:Vec<_> = (1..=n).map(|n| Some(n)).collect();
    let mut current_size = elves.len();
    let mut i = 0;

    while current_size != 1 {
        println!("csize={current_size}");
        if elves[i].is_some() {
            let mut j = (i + 1) % n;
            let mut todo = 0;
            loop {
                if elves[j].is_some() {
                    todo += 1;
                    if todo == current_size / 2 {
                        break;
                    }
                }
                
                j = (j + 1) % n;
            }
            elves[j] = None;
            current_size -= 1;
        }
        i = (i + 1) % n;
    }

    elves.iter().filter(|n| n.is_some()).nth(0).unwrap().unwrap()
}

/*
fn eliminate_fast(n: usize) -> usize {
    let mut elves:VecDeque<_> = (1..=n).collect();

    while elves.len() > 3 {
        let half = elves.len() / 2;
        let sub =  elves.len() / 3;
        let mut next_elves: VecDeque<_> = elves.clone().into_iter().take(sub).collect();
        let mut i = 0;
        
        while i < sub {
            if half % 2 == 1 {
                next_elves.push_back(i + half);
            } else {
                next_elves.push_back(i + half + 1);
            }            
            i += 2;
        }
        
        elves.rotate_left(sub);
    }

    match elves.len() {
        1 | 2 => {
            *elves.front().unwrap()
        }
        3 => {
            *elves.back().unwrap()
        }
        _ => {
            panic!("unexpected elements {}", elves.len());
        }
    }
}
*/

fn main() {
    println!("{}", eliminate_slow(3014603));
}
