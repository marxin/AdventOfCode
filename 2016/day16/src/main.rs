fn generate_dragon_code(input: Vec<bool>, length: usize) -> Vec<bool> {
    let mut code = input;

    while code.len() < length {
        let left = code.clone().into_iter();
        let right = code.into_iter().map(|c| !c).rev();
        code = left.chain([false].into_iter()).chain(right).collect();
    }

    code.into_iter().take(length).collect()
}

fn print_code(code: &Vec<bool>) {
    for c in code {
        print!("{}", *c as u8);
    }
    println!();
}

fn checksum(code: &Vec<bool>) -> Vec<bool> {
    let mut checksum = code.clone();

    while checksum.len() % 2 == 0 {
        checksum = checksum.chunks(2).map(|pair| pair[0] == pair[1]).collect();
    }

    checksum
}

fn main() {
    let key: Vec<bool> = "01000100010010111".chars().into_iter().map(|x| x == '1').collect();

    let code = generate_dragon_code(key, 35651584);
    // print_code(&code);
    print_code(&checksum(&code));
}
