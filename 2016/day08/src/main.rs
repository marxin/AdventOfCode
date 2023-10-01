use std::{fs, collections::VecDeque};

const WIDTH: usize = 50;
const HEIGHT: usize = 6;

fn main() {
    let mut board = [[false; HEIGHT]; WIDTH];
    
    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let parts: Vec<_> = line.split(&[' ', '=']).collect();
        let cmd = parts[0];
        match cmd {
            "rect" => {
                let dims: Vec<_> = parts[1].split(&['x']).collect();
                let w: usize = dims[0].parse().unwrap();
                let h: usize = dims[1].parse().unwrap();
                for x in 0..w {
                    board[x][0] = true;
                    board[x][h - 1] = true;
                }
                for y in 0..h {
                    board[0][y] = true;
                    board[w - 1][y] = true;
                }
            }
            "rotate" => {
                let what = parts[1];
                let nth: usize = parts[3].parse().unwrap();
                let by: usize = parts[5].parse().unwrap();
                match what {
                    "column" => {
                        let mut values = VecDeque::new();
                        for i in 0..HEIGHT {
                            values.push_back(board[nth][i]);
                        }
                        values.rotate_right(by);
                        for i in 0..HEIGHT {
                            board[nth][i] = values[i];
                        }
                    }
                    "row" => {
                        let mut values = VecDeque::new();
                        for i in 0..WIDTH {
                            values.push_back(board[i][nth]);
                        }
                        values.rotate_right(by);
                        for i in 0..WIDTH {
                            board[i][nth] = values[i];
                        }
                    }
                    _ => {
                        panic!("unexpected rotation {what}");
                    }
                }
            }
            _ =>  {
                panic!("unexpected command {cmd}");
            }
        }
    }

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            print!("{}", if board[x][y] { "#" } else {"."});
        }
        println!();
    }

    println!("{}", board.iter().flatten().filter(|&&x| x).count());
}
