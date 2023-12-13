use std::cmp::min;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn process_board(board: &[Vec<char>], res: &mut i64) {
    for start in 0..board.len() - 1 {
        let mut smudges = 0;
        for i in 0..min(start + 1, board.len() - start - 1) {
            for j in 0..board[0].len() {
                if board[start - i][j] != board[start + 1 + i][j] {
                    smudges += 1;
                }
            }
        }
        if smudges == 1 {
            *res += 100 * (start + 1) as i64;
            break;
        }
    }

    for start in 0..board[0].len() - 1 {
        let mut smudges = 0;
        for i in 0..min(start + 1, board[0].len() - start - 1) {
            for vec in board.iter() {
                if vec[start - i] != vec[start + 1 + i] {
                    smudges += 1;
                }
            }
        }
        if smudges == 1 {
            *res += (start + 1) as i64;
            break;
        }
    }
}

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut result: i64 = 0;

    let mut board: Vec<Vec<char>> = vec![];

    for line in reader.lines().map_while(Result::ok) {
        if line == *"" {
            process_board(&board, &mut result);
            board.clear();
        } else {
            board.push(line.chars().collect());
        }
    }

    process_board(&board, &mut result);

    println!("Final answer: {}", result);

    Ok(())
}
