use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut result = 0;

    let mut scheme :Vec<String> = vec![];

    for line in reader.lines() {
        if let Ok(content) = line {
            scheme.push(content);
        }
    }

    let re = Regex::new(r"[0-9]+").unwrap();

    for (line_nr, line) in scheme.iter().enumerate() {
        for m in re.find_iter(line) {
            let mut to_add = false;

            let start = m.start();
            let end = m.end();

            // ABOVE
            if line_nr > 0 {
                for i in start..end {
                    if scheme[line_nr - 1].chars().nth(i) != Some('.') && scheme[line_nr - 1].chars().nth(i).unwrap().to_digit(10).is_none() {
                        to_add = true;
                    }
                }
            }
            // BELOW
            if line_nr + 1 < scheme.len() {
                for i in start..end {
                    if scheme[line_nr + 1].chars().nth(i) != Some('.') && scheme[line_nr + 1].chars().nth(i).unwrap().to_digit(10).is_none() {
                        to_add = true;
                    }
                }
            }
            // LEFT
            if m.start() > 0 {
                if scheme[line_nr].chars().nth(start - 1) != Some('.') {
                    to_add = true;
                }
                if line_nr > 0 {
                    if scheme[line_nr - 1].chars().nth(start - 1) != Some('.') && scheme[line_nr - 1].chars().nth(start - 1).unwrap().to_digit(10).is_none() {
                        to_add = true;
                    }
                }
                if line_nr + 1 < scheme.len() {
                    if scheme[line_nr + 1].chars().nth(start - 1) != Some('.') && scheme[line_nr + 1].chars().nth(start - 1).unwrap().to_digit(10).is_none() {
                        to_add = true;
                    }
                }
            }

            // RIGHT
            if m.end() + 1 < scheme[line_nr].len() {
                if scheme[line_nr].chars().nth(end) != Some('.') {
                    to_add = true;
                }
                // assume all lines have equal length
                if line_nr > 0 {
                    if scheme[line_nr - 1].chars().nth(end) != Some('.') && scheme[line_nr - 1].chars().nth(end).unwrap().to_digit(10).is_none() {
                        to_add = true;
                    }
                }
                // as above
                if line_nr + 1 < scheme.len() {
                    if scheme[line_nr + 1].chars().nth(end) != Some('.') && scheme[line_nr + 1].chars().nth(end).unwrap().to_digit(10).is_none() {
                        to_add = true;
                    }
                }

            }

            if to_add {
                result += m.as_str().parse::<i32>().unwrap();
            }
        }
    }

    println!("Final answer: {}", result);

    Ok(())
}
