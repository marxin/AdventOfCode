use md5;

fn get_next_character(base: &str, start: u32) -> u32 {
    let mut i = start;
    loop {
        let candidate: String = format!("{base}{i}");
        let hash = md5::compute(candidate);
        let digest = format!("{hash:x}");
        if digest.starts_with("00000") {
            print!("{}", digest.chars().nth(5).unwrap());
            return i
        }
        i += 1;
    }
}

fn main() {
    let mut start: u32 = 0;
    for _ in 0..8 {
        start = get_next_character("ojvtpuvg", start);
        start += 1;
    }
    println!();
}
