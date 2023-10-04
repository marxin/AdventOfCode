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
            counter += nextn * times - buffer.len();
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
    assert_eq!(get_code_length(&String::from("A(2x2)BCD(2x2)EFG")), 11);
    assert_eq!(get_code_length(&String::from("X(8x2)(3x3)ABCY")), 18);
    assert_eq!(get_code_length(&String::from("(6x1)(1x3)A")), 6);

}
