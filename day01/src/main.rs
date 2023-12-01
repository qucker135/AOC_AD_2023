use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut result = 0;

    for line in reader.lines() {
        if let Ok(content) = line {
            let i1 = content.find(|c: char| c.is_digit(10));
            let i2 = content.rfind(|c: char| c.is_digit(10));

            if let (Some(i1), Some(i2)) = (i1, i2) {
                let num = 10 * content.chars().nth(i1).unwrap().to_digit(10).unwrap() + 
                    content.chars().nth(i2).unwrap().to_digit(10).unwrap();

                result += num;
            } 
        }
    }

    println!("Final answer: {}", result);

    Ok(())
}
