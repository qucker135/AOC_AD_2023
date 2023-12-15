use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn hash(string: &str) -> i64 {
    let mut res: i64 = 0;

    for ch in string.chars() {
        res += (ch as u32) as i64;
        res *= 17;
        res %= 256;
    }
    res
}

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut result: i64 = 0;

    let mut sequence: String = String::from("");

    for line in reader.lines().map_while(Result::ok) {
        sequence.push_str(line.trim());
    }

    for com in sequence.split(',') {
        result += hash(com);
    }

    println!("Final answer: {}", result);

    Ok(())
}
