use std::cmp::max;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn dfs(
    maze: &Vec<Vec<char>>,
    pos: (usize, usize),
    discovered: &mut Vec<(usize, usize)>,
    result: &mut usize,
) {
    // check if goal is reached
    if pos == (maze.len() - 1, maze[0].len() - 2) {
        *result = max(*result, discovered.len());
        return;
    }
    if !discovered.contains(&pos) {
        discovered.push(pos);
        let (y, x) = pos;
        if maze[y][x] == '>' && maze[y][x + 1] != '#' {
            dfs(maze, (y, x + 1), discovered, result);
        } else if maze[y][x] == '<' && maze[y][x - 1] != '#' {
            dfs(maze, (y, x - 1), discovered, result);
        } else if maze[y][x] == '^' && maze[y - 1][x] != '#' {
            dfs(maze, (y - 1, x), discovered, result);
        } else if maze[y][x] == 'v' && maze[y + 1][x] != '#' {
            dfs(maze, (y + 1, x), discovered, result);
        } else if maze[y][x] == '.' {
            if y > 0 && maze[y - 1][x] != '#' {
                dfs(maze, (y - 1, x), discovered, result);
            }
            if y + 1 < maze.len() && maze[y + 1][x] != '#' {
                dfs(maze, (y + 1, x), discovered, result);
            }
            if x > 0 && maze[y][x - 1] != '#' {
                dfs(maze, (y, x - 1), discovered, result);
            }
            if x + 1 < maze[y].len() && maze[y][x + 1] != '#' {
                dfs(maze, (y, x + 1), discovered, result);
            }
        }
        discovered.pop();
    }
}

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut result = 0;

    let mut maze: Vec<Vec<char>> = Vec::new();

    for line in reader.lines().map_while(Result::ok) {
        maze.push(line.chars().collect());
    }

    let mut discovered: Vec<(usize, usize)> = vec![];

    let start_pos: (usize, usize) = (0, 1);

    dfs(&maze, start_pos, &mut discovered, &mut result);

    println!("Final answer: {:?}", result);

    Ok(())
}
