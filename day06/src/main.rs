use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;
use std::iter::zip;

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let re = Regex::new(r"[0-9]+").unwrap();

    let mut result = 1;

    let mut times :Vec<i32> = vec![];
    let mut distances :Vec<i32> = vec![];

    for (line_nr, line) in reader.lines().enumerate() {
        if let Ok(content) = line {
            for m in re.find_iter(&content) {
                if content.starts_with("Time:") {
                    times.push(m.as_str().parse::<i32>().unwrap());
                }
                else if content.starts_with("Distance:") {
                    distances.push(m.as_str().parse::<i32>().unwrap());
                }
            }
        }
    }

    for (t, d) in zip(times.iter(), distances.iter()) {
        let mut sols = 0;
        for i in 0..t+1 {
            if i * (t - i) > *d {
                sols += 1;
            }
        }
        result *= sols;
    }

    println!("Final answer: {}", result);

    Ok(())
}
