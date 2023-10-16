fn get_next_row(input: Vec<char>) -> Vec<char> {
    let mut result = Vec::new();

    for (i, &center) in input.iter().enumerate() {
        let left = if i == 0 { '.' } else { input[i - 1] };
        let right = if i + 1 == input.len() { '.' } else { input[i + 1] };
        let r = match (left, center, right ) {
            ('^', '^', '.') | ('.', '^', '^' ) | ('^', '.', '.') | ('.', '.', '^') => { '^' }
            _ => { '.' }
        };
        result.push(r);
    }

    result
}

fn main() {
    let input_str = ".^^^.^.^^^^^..^^^..^..^..^^..^.^.^.^^.^^....^.^...^.^^.^^.^^..^^..^.^..^^^.^^...^...^^....^^.^^^^^^^";
    let mut input: Vec<_> = input_str.clone().chars().collect();
    let mut total = 0;

    for _ in 0..400000 {
        total += input.iter().filter(|&&c| c == '.').count();
        input = get_next_row(input);
    }

    println!("{total}");
}
