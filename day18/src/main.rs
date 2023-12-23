use std::cmp::{max, min};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut result: i64 = 0;

    let mut plan: Vec<(String, i64, String)> = vec![];

    let (mut y_min, mut y_max, mut x_min, mut x_max): (i64, i64, i64, i64) = (0, 0, 0, 0);

    let (mut y_cur, mut x_cur): (i64, i64) = (0, 0);

    for line in reader.lines().map_while(Result::ok) {
        let vec: Vec<&str> = line.split(' ').collect();
        let steps = vec[1].parse::<i64>().unwrap();
        plan.push((vec[0].to_string(), steps, vec[2].to_string()));
        if vec[0] == "U" {
            y_cur -= steps;
            y_min = min(y_min, y_cur);
        } else if vec[0] == "D" {
            y_cur += steps;
            y_max = max(y_max, y_cur);
        } else if vec[0] == "L" {
            x_cur -= steps;
            x_min = min(x_min, x_cur);
        } else if vec[0] == "R" {
            x_cur += steps;
            x_max = max(x_max, x_cur);
        }
    }

    let mut lagoon: Vec<Vec<char>> = vec![];

    for _ in 0..y_max - y_min + 1 {
        lagoon.push(vec!['.'; (x_max - x_min + 1) as usize]);
    }

    for com in plan.iter() {
        if com.0 == "U" {
            for _ in 1..=com.1 {
                y_cur -= 1;
                lagoon[(y_cur - y_min) as usize][(x_cur - x_min) as usize] = '#';
            }
        } else if com.0 == "D" {
            for _ in 1..=com.1 {
                y_cur += 1;
                lagoon[(y_cur - y_min) as usize][(x_cur - x_min) as usize] = '#';
            }
        } else if com.0 == "L" {
            for _ in 1..=com.1 {
                x_cur -= 1;
                lagoon[(y_cur - y_min) as usize][(x_cur - x_min) as usize] = '#';
            }
        } else if com.0 == "R" {
            for _ in 1..=com.1 {
                x_cur += 1;
                lagoon[(y_cur - y_min) as usize][(x_cur - x_min) as usize] = '#';
            }
        }
    }

    for row in &mut lagoon {
        if row[0] != '#' {
            row[0] = 'O';
        }
        if row.last() != Some(&'#') {
            let ind = row.len() - 1;
            row[ind] = 'O';
        }
    }

    for x in 0..lagoon[0].len() {
        if lagoon[0][x] != '#' {
            lagoon[0][x] = 'O';
        }
        if lagoon.last().unwrap()[x] != '#' {
            let ind = lagoon.len() - 1;
            lagoon[ind][x] = 'O';
        }
    }

    let mut change = true;

    while change {
        change = false;
        for y in 1..lagoon.len() - 1 {
            for x in 1..lagoon[y].len() - 1 {
                if lagoon[y][x] == '.'
                    && (lagoon[y - 1][x] == 'O'
                        || lagoon[y + 1][x] == 'O'
                        || lagoon[y][x - 1] == 'O'
                        || lagoon[y][x + 1] == 'O')
                {
                    lagoon[y][x] = 'O';
                    change = true;
                }
            }
        }
    }

    for row in lagoon.iter() {
        for ch in row.iter() {
            if ch == &'#' || ch == &'.' {
                result += 1;
            }
        }
    }

    println!("Final answer: {:?}", result);

    Ok(())
}
