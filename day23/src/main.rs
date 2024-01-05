use std::cmp::max;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn is_tile(tile: char) -> bool {
    ['.', '>', '<', 'v', '^'].contains(&tile)
}

fn get_vertices(maze: &[Vec<char>]) -> Vec<(usize, usize)> {
    let mut vertices: Vec<(usize, usize)> = Vec::new();

    for (line_nr, line) in maze.iter().enumerate() {
        for (char_nr, c) in line.iter().enumerate() {
            let mut adjacent_tiles = 0;
            if line_nr > 0 && is_tile(maze[line_nr - 1][char_nr]) {
                adjacent_tiles += 1;
            }
            if line_nr + 1 < maze.len() && is_tile(maze[line_nr + 1][char_nr]) {
                adjacent_tiles += 1;
            }
            if char_nr > 0 && is_tile(maze[line_nr][char_nr - 1]) {
                adjacent_tiles += 1;
            }
            if char_nr + 1 < maze[line_nr].len() && is_tile(maze[line_nr][char_nr + 1]) {
                adjacent_tiles += 1;
            }
            if is_tile(*c) && (line_nr == 0 || line_nr == maze.len() - 1 || adjacent_tiles > 2) {
                vertices.push((line_nr, char_nr));
            }
        }
    }

    vertices
}

fn get_edges(maze: &Vec<Vec<char>>) -> Vec<Vec<i64>> {
    let vertices = get_vertices(maze);

    let mut edges: Vec<Vec<i64>> = vec![vec![0; vertices.len()]; vertices.len()];

    for (v_id, vertex) in vertices.iter().enumerate() {
        let (y, x) = vertex;
        if *y > 0 && is_tile(maze[y - 1][*x]) {
            let (v_id2, edge_len) = find_edge(maze, &vertices, (y - 1, *x), Direction::Down, 1);
            if edges[v_id][v_id2] < edge_len {
                edges[v_id][v_id2] = edge_len;
            }
        }
        if y + 1 < maze.len() && is_tile(maze[y + 1][*x]) {
            let (v_id2, edge_len) = find_edge(maze, &vertices, (y + 1, *x), Direction::Up, 1);
            if edges[v_id][v_id2] < edge_len {
                edges[v_id][v_id2] = edge_len;
            }
        }
        if *x > 0 && is_tile(maze[*y][x - 1]) {
            let (v_id2, edge_len) = find_edge(maze, &vertices, (*y, x - 1), Direction::Right, 1);
            if edges[v_id][v_id2] < edge_len {
                edges[v_id][v_id2] = edge_len;
            }
        }
        if x + 1 < maze[*y].len() && is_tile(maze[*y][x + 1]) {
            let (v_id2, edge_len) = find_edge(maze, &vertices, (*y, x + 1), Direction::Left, 1);
            if edges[v_id][v_id2] < edge_len {
                edges[v_id][v_id2] = edge_len;
            }
        }
    }

    edges
}

fn find_edge(
    maze: &Vec<Vec<char>>,
    vertices: &Vec<(usize, usize)>,
    pos: (usize, usize),
    dir: Direction,
    edge_len: i64,
) -> (usize, i64) {
    if vertices.contains(&pos) {
        return (vertices.iter().position(|v| v == &pos).unwrap(), edge_len);
    }

    let (y, x) = pos;

    if y > 0 && is_tile(maze[y - 1][x]) && dir != Direction::Up {
        find_edge(maze, vertices, (y - 1, x), Direction::Down, edge_len + 1)
    } else if y + 1 < maze.len() && is_tile(maze[y + 1][x]) && dir != Direction::Down {
        find_edge(maze, vertices, (y + 1, x), Direction::Up, edge_len + 1)
    } else if x > 0 && is_tile(maze[y][x - 1]) && dir != Direction::Left {
        find_edge(maze, vertices, (y, x - 1), Direction::Right, edge_len + 1)
    } else if x + 1 < maze[y].len() && is_tile(maze[y][x + 1]) && dir != Direction::Right {
        find_edge(maze, vertices, (y, x + 1), Direction::Left, edge_len + 1)
    } else {
        panic!("No edge found");
    }
}

fn dfs_matrix(
    edges: &Vec<Vec<i64>>,
    pos: usize,
    discovered: &mut Vec<usize>,
    result: &mut i64,
    cur: i64,
) {
    if pos == edges.len() - 1 {
        *result = max(*result, cur);
        return;
    }
    if !discovered.contains(&pos) {
        discovered.push(pos);
        for (v_id, edge) in edges[pos].iter().enumerate() {
            if *edge > 0 {
                dfs_matrix(edges, v_id, discovered, result, cur + edge);
            }
        }
        discovered.pop();
    }
}

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut result: i64 = 0;

    let mut maze: Vec<Vec<char>> = vec![];

    for line in reader.lines().map_while(Result::ok) {
        maze.push(line.chars().collect());
    }

    let edges = get_edges(&maze);

    let mut discovered: Vec<usize> = vec![];

    dfs_matrix(&edges, 0, &mut discovered, &mut result, 0);

    println!("Final answer: {:?}", result);

    Ok(())
}
