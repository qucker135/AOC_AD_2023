#![feature(cmp_minmax)]
use std::cmp;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut result: i64 = 0;

    let mut universe: Vec<Vec<char>> = vec![];

    for line in reader.lines().map_while(Result::ok) {
        universe.push(line.chars().collect());
    }

    for (i1, vec1) in universe.iter().enumerate() {
        for (j1, ch1) in vec1.iter().enumerate() {
            for (i2, vec2) in universe.iter().enumerate() {
                for (j2, ch2) in vec2.iter().enumerate() {
                    if *ch1 == '#' && *ch2 == '#' && (i2 > i1 || (j2 > j1 && i1 == i2)) {
                        let mut add = (i2 as i64 - i1 as i64).abs() + (j1 as i64 - j2 as i64).abs();

                        let [i1_m, i2_m] = cmp::minmax(i1, i2);
                        let [j1_m, j2_m] = cmp::minmax(j1, j2);

                        // find empty rows
                        for vec in universe.iter().take(i2_m).skip(i1_m + 1) {
                            if vec.iter().all(|&x| x == '.') {
                                add += 999_999;
                            }
                        }

                        // find empty columns
                        for j in j1_m + 1..j2_m {
                            let mut empty = true;
                            for vec in universe.iter() {
                                if vec[j] == '#' {
                                    empty = false;
                                    break;
                                }
                            }
                            if empty {
                                add += 999_999;
                            }
                        }

                        result += add;
                    }
                }
            }
        }
    }

    println!("Final answer: {}", result);

    Ok(())
}
