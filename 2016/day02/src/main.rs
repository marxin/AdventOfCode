use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Could not open input file");
    let mut position:(i32, i32) = (0, 2);
    let moves:[(i32, i32);4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let move_names = "URDL";
    // let keypad = ["123", "456", "789"];
    let keypad = ["__1__", "_234_", "56789", "_ABC_", "__D__"];

    for line in input.lines() {
        // println!("Line: {}", line);

        for c in line.chars() {
            let m = &moves[move_names.find(c).unwrap()];
            let previous = position;

            position.0 += m.0;
            position.1 += m.1;

            if position.0 >= keypad[0].len() as i32 || position.0 < 0 || position.1 >= keypad.len() as i32 || position.1 < 0 {
                position = previous;
            }

            let key = keypad[position.1 as usize].chars().nth(position.0 as usize).unwrap();
            if key == '_' {
                position = previous;
            }
        }

        let key = keypad[position.1 as usize].chars().nth(position.0 as usize).unwrap();

        println!("{:?}: {}", position, key);
    }

    println!("Hello, world!");
}
