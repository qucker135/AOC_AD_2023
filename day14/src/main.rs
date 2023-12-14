use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut result: i64 = 0;

    let mut board: Vec<Vec<char>> = vec![];

    for line in reader.lines().map_while(Result::ok) {
        board.push(line.chars().collect());
    }

    for i in 1..board.len() {
        for j in 0..board[0].len() {
            if board[i][j] == 'O' {
                let mut pos = i;
                while pos > 0 && board[pos - 1][j] == '.' {
                    board[pos][j] = '.';
                    board[pos - 1][j] = 'O';
                    pos -= 1;
                }
            }
        }
    }

    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if board[i][j] == 'O' {
                result += (board.len() - i) as i64;
            }
        }
    }

    println!("Final answer: {}", result);

    Ok(())
}
