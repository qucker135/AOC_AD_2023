#![feature(iter_array_chunks)]

use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let re = Regex::new(r"[0-9]+").unwrap();

    let mut result :i64 = i64::MAX;

    let mut seeds :Vec<[i64; 2]> = vec![];
    
    let mut maps :Vec<Vec<[i64; 3]>> = vec![];

    for line in reader.lines() {
        if let Ok(content) = line {
            // read seeds numbers
            if content.starts_with("seeds: ") {
                for [m1, m2] in re.find_iter(&content).array_chunks() {
                    seeds.push(
                        [
                            m1.as_str().parse::<i64>().unwrap(),
                            m2.as_str().parse::<i64>().unwrap()
                        ]
                    );
                }
            }
            // create empty map
            else if content.ends_with("map:") {
                maps.push(Vec::new());
            }
            // read maps
            else if content.len() > 0 {
                let arr :[i64; 3] = content.split(" ").map(|s: &str| s.parse::<i64>().unwrap()).collect::<Vec<i64>>().try_into().unwrap();
                let last = maps.len() - 1;
                maps[last].push(arr);
            }
        }
    }
    
    // bruteforce solution, probably not the most elegant one
    for seed_range in seeds.iter() {
        for seed in seed_range[0]..seed_range[0]+seed_range[1] {
            let mut value :i64 = seed;
            for map in maps.iter() {
                for entry in map.iter() {
                    if (value >= entry[1]) && (value < entry[1] + entry[2]) {
                        value = entry[0] + value - entry[1];
                        break;
                    }
                }
            }
            if value < result {
                result = value;
            }
        }
    }

    println!("Final answer: {}", result);

    Ok(())
}
