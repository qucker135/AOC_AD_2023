use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn trace_rays(
    contraption: &[Vec<char>],
    directions: &mut [Vec<HashSet<(Direction, Direction)>>],
    y: usize,
    x: usize,
    in_dir: Direction,
) {
    let mut out_dirs: Vec<Direction> = vec![];
    if contraption[y][x] == '.' {
        out_dirs.push(in_dir);
    } else if contraption[y][x] == '/' {
        match in_dir {
            Direction::Left => out_dirs.push(Direction::Down),
            Direction::Right => out_dirs.push(Direction::Up),
            Direction::Up => out_dirs.push(Direction::Right),
            Direction::Down => out_dirs.push(Direction::Left),
        };
    } else if contraption[y][x] == '\\' {
        match in_dir {
            Direction::Left => out_dirs.push(Direction::Up),
            Direction::Right => out_dirs.push(Direction::Down),
            Direction::Up => out_dirs.push(Direction::Left),
            Direction::Down => out_dirs.push(Direction::Right),
        };
    } else if contraption[y][x] == '-' {
        match in_dir {
            Direction::Left | Direction::Right => out_dirs.push(in_dir),
            _ => {
                out_dirs.push(Direction::Left);
                out_dirs.push(Direction::Right);
            }
        };
    } else if contraption[y][x] == '|' {
        match in_dir {
            Direction::Up | Direction::Down => out_dirs.push(in_dir),
            _ => {
                out_dirs.push(Direction::Up);
                out_dirs.push(Direction::Down);
            }
        };
    }
    for out_dir in out_dirs {
        if directions[y][x].insert((in_dir, out_dir)) {
            match out_dir {
                Direction::Left => {
                    if x > 0 {
                        trace_rays(contraption, directions, y, x - 1, out_dir);
                    }
                }
                Direction::Right => {
                    if x + 1 < contraption[y].len() {
                        trace_rays(contraption, directions, y, x + 1, out_dir);
                    }
                }
                Direction::Up => {
                    if y > 0 {
                        trace_rays(contraption, directions, y - 1, x, out_dir);
                    }
                }
                Direction::Down => {
                    if y + 1 < contraption.len() {
                        trace_rays(contraption, directions, y + 1, x, out_dir);
                    }
                }
            }
        }
    }
}

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut result: i64 = 0;

    let mut contraption: Vec<Vec<char>> = vec![];

    let mut directions_orig: Vec<Vec<HashSet<(Direction, Direction)>>> = vec![];

    for line in reader.lines().map_while(Result::ok) {
        contraption.push(line.chars().collect());
        directions_orig.push(vec![HashSet::new(); line.chars().count()]);
    }

    for i in 0..contraption.len() {
        let mut directions = directions_orig.clone();
        let mut tmp_result = 0;

        trace_rays(&contraption, &mut directions, i, 0, Direction::Right);

        for vec in directions.iter() {
            for h in vec.iter() {
                if !h.is_empty() {
                    tmp_result += 1;
                }
            }
        }
        if tmp_result > result {
            result = tmp_result;
        }
    }
    for i in 0..contraption.len() {
        let mut directions = directions_orig.clone();
        let mut tmp_result = 0;

        trace_rays(
            &contraption,
            &mut directions,
            i,
            contraption[0].len() - 1,
            Direction::Left,
        );

        for vec in directions.iter() {
            for h in vec.iter() {
                if !h.is_empty() {
                    tmp_result += 1;
                }
            }
        }
        if tmp_result > result {
            result = tmp_result;
        }
    }
    for i in 0..contraption[0].len() {
        let mut directions = directions_orig.clone();
        let mut tmp_result = 0;

        trace_rays(&contraption, &mut directions, 0, i, Direction::Down);

        for vec in directions.iter() {
            for h in vec.iter() {
                if !h.is_empty() {
                    tmp_result += 1;
                }
            }
        }
        if tmp_result > result {
            result = tmp_result;
        }
    }
    for i in 0..contraption[0].len() {
        let mut directions = directions_orig.clone();
        let mut tmp_result = 0;

        trace_rays(
            &contraption,
            &mut directions,
            contraption.len() - 1,
            i,
            Direction::Up,
        );

        for vec in directions.iter() {
            for h in vec.iter() {
                if !h.is_empty() {
                    tmp_result += 1;
                }
            }
        }
        if tmp_result > result {
            result = tmp_result;
        }
    }

    println!("Final answer: {}", result);

    Ok(())
}
