use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;
use regex::Regex;

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut result = 0;

    let re = Regex::new(r"[0-9]+").unwrap();

    for line in reader.lines() {
        if let Ok(content) = line {
            let sets_strings :Vec<&str> = content.split(':').collect::<Vec<&str>>()[1].split('|').collect();

            let mut h0 :HashSet<i32> = HashSet::new();
            let mut h1 :HashSet<i32> = HashSet::new();

            for i in re.find_iter(sets_strings[0]) {
                h0.insert(i.as_str().parse::<i32>().unwrap());
            }

            for i in re.find_iter(sets_strings[1]) {
                h1.insert(i.as_str().parse::<i32>().unwrap());
            }

            let length = h0.intersection(&h1).collect::<HashSet<_>>().len();

            if length > 0 {
                result += 2_i32.pow((length - 1).try_into().unwrap());
            }
        }
    }

    
    println!("Final answer: {}", result);

    Ok(())
}
