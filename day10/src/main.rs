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

    let mut maze_interpolated: Vec<Vec<char>> =
        vec![vec!['I'; maze.len() * 2 - 1]; maze[0].len() * 2 - 1];

    maze_interpolated[cur_pos.0 * 2][cur_pos.1 * 2] = 'B';

    if cur_pos.0 + 1 < maze.len() && ['|', 'J', 'L'].contains(&maze[cur_pos.0 + 1][cur_pos.1]) {
        maze_interpolated[cur_pos.0 * 2 + 1][cur_pos.1 * 2] = 'B';
        maze_interpolated[cur_pos.0 * 2 + 2][cur_pos.1 * 2] = 'B';
        cur_pos.0 += 1;
        last_move = Move::Down;
    } else if cur_pos.0 > 0 && ['|', 'F', '7'].contains(&maze[cur_pos.0 - 1][cur_pos.1]) {
        maze_interpolated[cur_pos.0 * 2 - 1][cur_pos.1 * 2] = 'B';
        maze_interpolated[cur_pos.0 * 2 - 2][cur_pos.1 * 2] = 'B';
        cur_pos.0 -= 1;
        last_move = Move::Up;
    } else if cur_pos.1 + 1 < maze[cur_pos.0].len()
        && ['-', 'J', '7'].contains(&maze[cur_pos.0][cur_pos.1 + 1])
    {
        maze_interpolated[cur_pos.0 * 2][cur_pos.1 * 2 + 1] = 'B';
        maze_interpolated[cur_pos.0 * 2][cur_pos.1 * 2 + 2] = 'B';
        cur_pos.1 += 1;
        last_move = Move::Right;
    } else if cur_pos.1 > 0 && ['-', 'F', 'L'].contains(&maze[cur_pos.0][cur_pos.1 - 1]) {
        maze_interpolated[cur_pos.0 * 2][cur_pos.1 * 2 - 1] = 'B';
        maze_interpolated[cur_pos.0 * 2][cur_pos.1 * 2 - 2] = 'B';
        cur_pos.1 -= 1;
        last_move = Move::Left;
    }

    while maze[cur_pos.0][cur_pos.1] != 'S' {
        if maze[cur_pos.0][cur_pos.1] == '|' && last_move == Move::Down {
            maze_interpolated[cur_pos.0 * 2 + 1][cur_pos.1 * 2] = 'B';
            maze_interpolated[cur_pos.0 * 2 + 2][cur_pos.1 * 2] = 'B';
            cur_pos.0 += 1;
            last_move = Move::Down;
        } else if maze[cur_pos.0][cur_pos.1] == 'J' && last_move == Move::Down {
            maze_interpolated[cur_pos.0 * 2][cur_pos.1 * 2 - 1] = 'B';
            maze_interpolated[cur_pos.0 * 2][cur_pos.1 * 2 - 2] = 'B';
            cur_pos.1 -= 1;
            last_move = Move::Left;
        } else if maze[cur_pos.0][cur_pos.1] == 'L' && last_move == Move::Down {
            maze_interpolated[cur_pos.0 * 2][cur_pos.1 * 2 + 1] = 'B';
            maze_interpolated[cur_pos.0 * 2][cur_pos.1 * 2 + 2] = 'B';
            cur_pos.1 += 1;
            last_move = Move::Right;
        } else if maze[cur_pos.0][cur_pos.1] == '|' && last_move == Move::Up {
            maze_interpolated[cur_pos.0 * 2 - 1][cur_pos.1 * 2] = 'B';
            maze_interpolated[cur_pos.0 * 2 - 2][cur_pos.1 * 2] = 'B';
            cur_pos.0 -= 1;
            last_move = Move::Up;
        } else if maze[cur_pos.0][cur_pos.1] == '7' && last_move == Move::Up {
            maze_interpolated[cur_pos.0 * 2][cur_pos.1 * 2 - 1] = 'B';
            maze_interpolated[cur_pos.0 * 2][cur_pos.1 * 2 - 2] = 'B';
            cur_pos.1 -= 1;
            last_move = Move::Left;
        } else if maze[cur_pos.0][cur_pos.1] == 'F' && last_move == Move::Up {
            maze_interpolated[cur_pos.0 * 2][cur_pos.1 * 2 + 1] = 'B';
            maze_interpolated[cur_pos.0 * 2][cur_pos.1 * 2 + 2] = 'B';
            cur_pos.1 += 1;
            last_move = Move::Right;
        } else if maze[cur_pos.0][cur_pos.1] == '-' && last_move == Move::Left {
            maze_interpolated[cur_pos.0 * 2][cur_pos.1 * 2 - 1] = 'B';
            maze_interpolated[cur_pos.0 * 2][cur_pos.1 * 2 - 2] = 'B';
            cur_pos.1 -= 1;
            last_move = Move::Left;
        } else if maze[cur_pos.0][cur_pos.1] == 'F' && last_move == Move::Left {
            maze_interpolated[cur_pos.0 * 2 + 1][cur_pos.1 * 2] = 'B';
            maze_interpolated[cur_pos.0 * 2 + 2][cur_pos.1 * 2] = 'B';
            cur_pos.0 += 1;
            last_move = Move::Down;
        } else if maze[cur_pos.0][cur_pos.1] == 'L' && last_move == Move::Left {
            maze_interpolated[cur_pos.0 * 2 - 1][cur_pos.1 * 2] = 'B';
            maze_interpolated[cur_pos.0 * 2 - 2][cur_pos.1 * 2] = 'B';
            cur_pos.0 -= 1;
            last_move = Move::Up;
        } else if maze[cur_pos.0][cur_pos.1] == '-' && last_move == Move::Right {
            maze_interpolated[cur_pos.0 * 2][cur_pos.1 * 2 + 1] = 'B';
            maze_interpolated[cur_pos.0 * 2][cur_pos.1 * 2 + 2] = 'B';
            cur_pos.1 += 1;
            last_move = Move::Right;
        } else if maze[cur_pos.0][cur_pos.1] == '7' && last_move == Move::Right {
            maze_interpolated[cur_pos.0 * 2 + 1][cur_pos.1 * 2] = 'B';
            maze_interpolated[cur_pos.0 * 2 + 2][cur_pos.1 * 2] = 'B';
            cur_pos.0 += 1;
            last_move = Move::Down;
        } else if maze[cur_pos.0][cur_pos.1] == 'J' && last_move == Move::Right {
            maze_interpolated[cur_pos.0 * 2 - 1][cur_pos.1 * 2] = 'B';
            maze_interpolated[cur_pos.0 * 2 - 2][cur_pos.1 * 2] = 'B';
            cur_pos.0 -= 1;
            last_move = Move::Up;
        }
    }

    for v in &mut maze_interpolated {
        if v[0] != 'B' {
            v[0] = 'O';
        }
        let last_ind = v.len() - 1;
        if v[last_ind] != 'B' {
            v[last_ind] = 'O';
        }
    }

    for i in 0..maze_interpolated[0].len() {
        if maze_interpolated[0][i] != 'B' {
            maze_interpolated[0][i] = 'O';
        }
        if maze_interpolated[maze.len() * 2 - 2][i] != 'B' {
            maze_interpolated[maze.len() * 2 - 2][i] = 'O';
        }
    }

    let mut state_mutated = true;

    while state_mutated {
        state_mutated = false;
        for i in 1..maze_interpolated.len() - 1 {
            for j in 1..maze_interpolated[i].len() - 1 {
                if (maze_interpolated[i - 1][j] == 'O'
                    || maze_interpolated[i + 1][j] == 'O'
                    || maze_interpolated[i][j - 1] == 'O'
                    || maze_interpolated[i][j + 1] == 'O')
                    && maze_interpolated[i][j] == 'I'
                {
                    maze_interpolated[i][j] = 'O';
                    state_mutated = true;
                }
            }
        }
    }

    for i in 0..maze.len() {
        for j in 0..maze[i].len() {
            if maze_interpolated[2 * i][2 * j] == 'I' {
                result += 1;
            }
        }
    }

    println!("Final answer: {}", result);

    Ok(())
}
