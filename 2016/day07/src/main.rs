use std::fs;

fn supports_tls(packet: &str) -> bool {
    let mut in_mask = false;
    let mut found = false;
    let data: Vec<char> = packet.chars().collect();

    for index in 0..data.len() - 3 {
        let c = data[index];
        match c {
            '[' => {in_mask = true; continue; },
            ']' => {in_mask = false; continue; }
            _ => {}
        }

        if data[index] == data[index + 3] &&
            data[index + 1] == data[index + 2] &&
            data[index] != data[index + 1] {
            if in_mask {
                return false;
            } else {
                found = true;
            }
        }
    }

    found
}

fn main() {
    assert_eq!(supports_tls("abba[mnop]qrst"), true);
    assert_eq!(supports_tls("abcd[bddb]xyyx"), false);
    assert_eq!(supports_tls("aaaa[qwer]tyui"), false);
    assert_eq!(supports_tls("ioxxoj[asdfgh]zxcvbn"), true);

    let mut counter = 0;

    for line in fs::read_to_string("input.txt").unwrap().lines() {
        if supports_tls(line) {
            counter += 1;
        }
    }

    println!("{counter}");
}
