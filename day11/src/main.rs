use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut result = 0;

    let mut universe: Vec<Vec<char>> = vec![];

    for line in reader.lines().map_while(Result::ok) {
        universe.push(line.chars().collect());
    }

    // double each empty row

    let mut indices: Vec<usize> = vec![];

    for (i, v) in universe.iter().enumerate() {
        if v.iter().all(|&x| x == '.') {
            indices.push(i);
        }
    }

    let width = universe[0].len();
    for ind in indices.iter().rev() {
        universe.insert(*ind, vec!['.'; width]);
    }

    // double each empty column

    let mut indices: Vec<usize> = vec![];

    for j in 0..universe[0].len() {
        let mut empty = true;
        for vec in universe.iter() {
            if vec[j] == '#' {
                empty = false;
                break;
            }
        }
        if empty {
            indices.push(j);
        }
    }

    for ind in indices.iter().rev() {
        for vec in universe.iter_mut() {
            vec.insert(*ind, '.');
        }
    }

    for (i1, vec1) in universe.iter().enumerate() {
        for (j1, ch1) in vec1.iter().enumerate() {
            for (i2, vec2) in universe.iter().enumerate() {
                for (j2, ch2) in vec2.iter().enumerate() {
                    if *ch1 == '#' && *ch2 == '#' && (i2 > i1 || (j2 > j1 && i1 == i2)) {
                        let add = (i2 as i32 - i1 as i32).abs() + (j1 as i32 - j2 as i32).abs();
                        result += add;
                    }
                }
            }
        }
    }

    println!("Final answer: {}", result);

    Ok(())
}
