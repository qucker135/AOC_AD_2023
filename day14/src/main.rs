use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn cycle_board(board: &mut [Vec<char>]) {
    // NORTH
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
    // WEST
    for i in 0..board.len() {
        for j in 1..board[0].len() {
            if board[i][j] == 'O' {
                let mut pos = j;
                while pos > 0 && board[i][pos - 1] == '.' {
                    board[i][pos] = '.';
                    board[i][pos - 1] = 'O';
                    pos -= 1;
                }
            }
        }
    }
    // SOUTH
    for i in (0..board.len() - 1).rev() {
        for j in 0..board[0].len() {
            if board[i][j] == 'O' {
                let mut pos = i;
                while pos + 1 < board.len() && board[pos + 1][j] == '.' {
                    board[pos][j] = '.';
                    board[pos + 1][j] = 'O';
                    pos += 1;
                }
            }
        }
    }
    // EAST
    for i in 0..board.len() {
        for j in (0..board[0].len() - 1).rev() {
            if board[i][j] == 'O' {
                let mut pos = j;
                while pos + 1 < board[0].len() && board[i][pos + 1] == '.' {
                    board[i][pos] = '.';
                    board[i][pos + 1] = 'O';
                    pos += 1;
                }
            }
        }
    }
}

fn compute_hashes(board: &[Vec<char>]) -> Vec<u8> {
    let mut hashes: Vec<u8> = vec![];

    for row in board.iter() {
        let mut hash: u8 = 0;
        for ch in row.iter() {
            if *ch == 'O' {
                hash += 1;
            }
        }
        hashes.push(hash);
    }
    hashes
}

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut result: i64 = 0;

    let mut board: Vec<Vec<char>> = vec![];

    for line in reader.lines().map_while(Result::ok) {
        board.push(line.chars().collect());
    }

    const BILLION: usize = 1_000_000_000;

    let board_orig = board.clone();

    let mut hashes_vec: Vec<Vec<u8>> = vec![];

    hashes_vec.push(compute_hashes(&board));

    'main: for c in 1..=BILLION {
        cycle_board(&mut board);

        let hashes = compute_hashes(&board);

        if let Some(ind) = hashes_vec.iter().position(|h| *h == hashes) {
            // hashes correct, need to check boards
            let mut board1 = board_orig.clone();

            for _i in 0..ind {
                cycle_board(&mut board1);
            }

            let mut board2 = board1.clone();

            for _i in 0..c - ind {
                cycle_board(&mut board2);
            }

            if board1 == board2 {
                // cycle found!
                let cycle = c - ind;

                let mut it = BILLION % cycle;
                while it < ind {
                    it += cycle;
                }

                board = board_orig.clone();

                for _i in 0..it {
                    cycle_board(&mut board);
                }
                break 'main;
            }
        } else {
            hashes_vec.push(compute_hashes(&board));
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
