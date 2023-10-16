fn main() {
    let mut elves:Vec<_> = (1..=3014603).collect();

    while elves.len() != 1 {
        let odd = elves.len() % 2 == 1;
        elves = elves.into_iter().step_by(2).collect();
        if odd {
            elves.remove(0);
        }
    }

    println!("{}", elves.first().unwrap());
}
