use md5;

const DIGITS: usize = 8;

fn get_next_character(base: &str, start: u32, digits: & mut[char; DIGITS]) -> u32 {
    let mut i = start;
    loop {
        let candidate: String = format!("{base}{i}");
        let hash = md5::compute(candidate);
        let digest = format!("{hash:x}");
        if digest.starts_with("00000") {
            let pos = String::from_iter(digest.chars().skip(5).take(1));
            let index = u32::from_str_radix(pos.as_str(), 16).unwrap() as usize;
            println!("{digest} {index}");
            if index < DIGITS && digits[index] == '_' {
                let value = digest.chars().nth(6).unwrap();
                digits[index] = value;
                println!("[{index}] = {value}");
            }
            return i
        }
        i += 1;
    }
}

fn main() {
    let mut start: u32 = 0;
    let mut digits: [char; DIGITS] = ['_'; DIGITS];

    loop {
        start = get_next_character("ojvtpuvg", start, &mut digits);
        start += 1;
        println!("{:?}", digits);
        if digits.iter().all(|x| x != &'_') {
            println!("{:?}", digits);
            break;
        }

    }
    println!("{}", String::from_iter(digits));
}
