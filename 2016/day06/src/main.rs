use std::collections::HashMap;
use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();

    for i in 0..8 {
        let mut dictionary = HashMap::new();
        for line in data.lines() {
            let c = line.chars().nth(i).unwrap();
            if dictionary.contains_key(&c) {
                *dictionary.get_mut(&c).unwrap() += 1;
            }
            else {
                dictionary.insert(c, 1);
            }
        }

        let most = dictionary.iter().max_by_key(|x| x.1).unwrap().0;
        print!("{}", most);
    }

    println!();
}
