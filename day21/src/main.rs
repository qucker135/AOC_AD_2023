use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut result: i64 = 0;

    let mut garden: Vec<Vec<char>> = vec![];
    let mut distances: Vec<Vec<Option<i64>>> = vec![];

    for line in reader.lines().map_while(Result::ok) {
        garden.push(line.chars().collect());
        distances.push(vec![None; line.len()]);
    }

    'out: for (y, row) in garden.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 'S' {
                distances[y][x] = Some(0);
                break 'out;
            }
        }
    }

    let mut dist = 1;

    let mut change = true;

    while change {
        change = false;

        for (y, row) in garden.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == '#' {
                    continue;
                }

                if y > 0 && distances[y - 1][x] == Some(dist - 1) && distances[y][x].is_none() {
                    distances[y][x] = Some(dist);
                    change = true;
                }

                if x > 0 && distances[y][x - 1] == Some(dist - 1) && distances[y][x].is_none() {
                    distances[y][x] = Some(dist);
                    change = true;
                }

                if y + 1 < garden.len()
                    && distances[y + 1][x] == Some(dist - 1)
                    && distances[y][x].is_none()
                {
                    distances[y][x] = Some(dist);
                    change = true;
                }

                if x + 1 < row.len()
                    && distances[y][x + 1] == Some(dist - 1)
                    && distances[y][x].is_none()
                {
                    distances[y][x] = Some(dist);
                    change = true;
                }
            }
        }
        dist += 1;
    }

    for row in distances.iter() {
        for distance in row.iter().flatten() {
            if distance % 2 == 0 && *distance <= 64 {
                result += 1;
            }
        }
    }

    println!("Final answer: {:?}", result);

    Ok(())
}
