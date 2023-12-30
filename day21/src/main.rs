use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut garden: Vec<Vec<char>> = vec![];
    let mut distances: Vec<Vec<Option<i64>>> = vec![];

    for line in reader.lines().map_while(Result::ok) {
        garden.push(
            line.chars()
                .map(|c| if c == 'S' { '.' } else { c })
                .collect(),
        );
    }

    garden = garden
        .iter()
        .cycle()
        .take(garden.len() * 5)
        .map(|x| {
            (*x).clone()
                .iter()
                .cycle()
                .take(x.len() * 5)
                .copied()
                .collect()
        })
        .collect();

    for row in garden.iter() {
        distances.push(vec![None; row.len()]);
    }

    let start_pos: (usize, usize) = (garden.len() / 2, garden[0].len() / 2);

    distances[start_pos.0][start_pos.1] = Some(0);

    for dist in 1..=65 + 2 * 131 {
        for (y, row) in garden.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == '#' {
                    continue;
                }

                if y > 0 && distances[y - 1][x] == Some(dist - 1) && distances[y][x].is_none() {
                    distances[y][x] = Some(dist);
                }

                if x > 0 && distances[y][x - 1] == Some(dist - 1) && distances[y][x].is_none() {
                    distances[y][x] = Some(dist);
                }

                if y + 1 < garden.len()
                    && distances[y + 1][x] == Some(dist - 1)
                    && distances[y][x].is_none()
                {
                    distances[y][x] = Some(dist);
                }

                if x + 1 < row.len()
                    && distances[y][x + 1] == Some(dist - 1)
                    && distances[y][x].is_none()
                {
                    distances[y][x] = Some(dist);
                }
            }
        }
    }

    // values of quadratic formula a*n^2 + b*n + c for n = {0, 1, 2}
    let mut ys: [i64; 3] = [0, 0, 0];

    for row in distances.iter() {
        for distance in row.iter().flatten() {
            for i in 0..3 {
                if distance % 2 == (65 + i * 131) % 2 && *distance <= 65 + i * 131 {
                    ys[i as usize] += 1;
                }
            }
        }
    }

    let c = ys[0];
    let a = (ys[2] + ys[0]) / 2 - ys[1];
    let b = ys[1] - a - c;

    // 26501365 = 65 + 131 * 202300
    let result: i64 = a * 202300i64.pow(2) + b * 202300i64 + c;

    println!("Final answer: {:?}", result);

    Ok(())
}
