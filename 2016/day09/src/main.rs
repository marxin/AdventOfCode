use std::fs;

fn get_code_length(data: &str) -> usize {
    let mut buffer = String::from("");

    let vector: Vec<_> = data.chars().collect();
    let mut i = 0;
    let mut counter = 0;

    while i < vector.len() {
        let c = vector[i];
        if c == '(' { 
            buffer = String::from("");
        } else if c == ')' {
            let parts: Vec<_> = buffer.split(&['x']).collect();
            let nextn: usize = parts[0].parse().unwrap();
            let times: usize = parts[1].parse().unwrap();
            let content = String::from_iter(vector.iter().skip(i + 1).take(nextn));
            let child_len = get_code_length(&content);
            counter += times * child_len;
            counter -= buffer.len();
            i += nextn;
        } else {
            counter += 1;
            buffer.push(c);
        }

        i += 1;
    }

    counter
}

fn main() {
    println!("{}", get_code_length(&fs::read_to_string("input.txt").unwrap().trim()));
    assert_eq!(get_code_length(&String::from("ADVENT")), 6);
    assert_eq!(get_code_length(&String::from("(27x12)(20x12)(13x14)(7x10)(1x12)A")), 241920);
    assert_eq!(get_code_length(&String::from("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN")), 445);
}
