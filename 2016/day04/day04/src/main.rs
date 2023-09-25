use std::{fs, str::FromStr};

const ALPHABET_SIZE: u32 = 'z' as u32 - 'a' as u32 + 1;

fn main() {
    let content = fs::read_to_string("input.txt").expect("Could not open the input file!");
    for line in content.lines() {
        let parts: Vec<_> = line.split(&['[', '-']).collect();
        let cypher: u32 = parts[parts.len() - 2].parse().unwrap();
        let mut output: String = String::from_str("").unwrap();
        for c in line.chars() {
            if c.is_digit(10) {
                break;
            }
            let mut decyphered = ' ';
            if c != '-' {
                let ord: u32 = (c as u32 - 'a' as u32 + cypher) % ALPHABET_SIZE;
                decyphered = char::from_u32('a' as u32 + ord).unwrap();
            }
            output.push(decyphered);
        }

        if output.contains("northpole object storage") {
            println!("{output} [{cypher}]");
        }
    }
}
