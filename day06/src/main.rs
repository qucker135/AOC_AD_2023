use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let re = Regex::new(r"[0-9]+").unwrap();

    let mut result = 0;

    let mut time :String = "".to_string();
    let mut distance :String = "".to_string();

    for line in reader.lines() {
        if let Ok(content) = line {
            for m in re.find_iter(&content) {
                if content.starts_with("Time:") {
                    time.push_str(m.as_str());
                }
                else if content.starts_with("Distance:") {
                    distance.push_str(m.as_str());
                }
            }
        }
    }

    let time :i64 = time.parse::<i64>().unwrap();
    let distance :i64 = distance.parse::<i64>().unwrap();

    for i in 0..time+1 {
        if i * (time - i) > distance {
            result += 1;
        }
    }

    println!("Final answer: {}", result);

    Ok(())
}
