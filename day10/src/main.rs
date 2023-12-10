use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug, PartialEq)]
enum Move {
    Left,
    Right,
    Up,
    Down,
}

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut result = 0;

    let mut maze: Vec<Vec<char>> = vec![];

    for line in reader.lines().map_while(Result::ok) {
        maze.push(line.chars().collect());
    }

    let mut cur_pos: (usize, usize) = (0, 0);

    let mut last_move = Move::Down;

    'outer: for (line_nr, line) in maze.iter().enumerate() {
        for (ch_nr, ch) in line.iter().enumerate() {
            if *ch == 'S' {
                cur_pos = (line_nr, ch_nr);
                break 'outer;
            }
        }
    }

    if cur_pos.0 + 1 < maze.len() && ['|', 'J', 'L'].contains(&maze[cur_pos.0 + 1][cur_pos.1]) {
        cur_pos.0 += 1;
        last_move = Move::Down;
    } else if cur_pos.0 > 0 && ['|', 'F', '7'].contains(&maze[cur_pos.0 - 1][cur_pos.1]) {
        cur_pos.0 -= 1;
        last_move = Move::Up;
    } else if cur_pos.1 + 1 < maze[cur_pos.0].len()
        && ['-', 'J', '7'].contains(&maze[cur_pos.0][cur_pos.1 + 1])
    {
        cur_pos.1 += 1;
        last_move = Move::Right;
    } else if cur_pos.1 > 0 && ['-', 'F', 'L'].contains(&maze[cur_pos.0][cur_pos.1 - 1]) {
        cur_pos.1 -= 1;
        last_move = Move::Left;
    }
    result += 1;

    while maze[cur_pos.0][cur_pos.1] != 'S' {
        if maze[cur_pos.0][cur_pos.1] == '|' && last_move == Move::Down {
            cur_pos.0 += 1;
            last_move = Move::Down;
        } else if maze[cur_pos.0][cur_pos.1] == 'J' && last_move == Move::Down {
            cur_pos.1 -= 1;
            last_move = Move::Left;
        } else if maze[cur_pos.0][cur_pos.1] == 'L' && last_move == Move::Down {
            cur_pos.1 += 1;
            last_move = Move::Right;
        } else if maze[cur_pos.0][cur_pos.1] == '|' && last_move == Move::Up {
            cur_pos.0 -= 1;
            last_move = Move::Up;
        } else if maze[cur_pos.0][cur_pos.1] == '7' && last_move == Move::Up {
            cur_pos.1 -= 1;
            last_move = Move::Left;
        } else if maze[cur_pos.0][cur_pos.1] == 'F' && last_move == Move::Up {
            cur_pos.1 += 1;
            last_move = Move::Right;
        } else if maze[cur_pos.0][cur_pos.1] == '-' && last_move == Move::Left {
            cur_pos.1 -= 1;
            last_move = Move::Left;
        } else if maze[cur_pos.0][cur_pos.1] == 'F' && last_move == Move::Left {
            cur_pos.0 += 1;
            last_move = Move::Down;
        } else if maze[cur_pos.0][cur_pos.1] == 'L' && last_move == Move::Left {
            cur_pos.0 -= 1;
            last_move = Move::Up;
        } else if maze[cur_pos.0][cur_pos.1] == '-' && last_move == Move::Right {
            cur_pos.1 += 1;
            last_move = Move::Right;
        } else if maze[cur_pos.0][cur_pos.1] == '7' && last_move == Move::Right {
            cur_pos.0 += 1;
            last_move = Move::Down;
        } else if maze[cur_pos.0][cur_pos.1] == 'J' && last_move == Move::Right {
            cur_pos.0 -= 1;
            last_move = Move::Up;
        }
        result += 1;
    }

    result /= 2;

    println!("Final answer: {}", result);

    Ok(())
}
