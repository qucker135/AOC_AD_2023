use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let result :i64;

    let re = Regex::new(r"[0-9]+").unwrap();

    let mut seeds :Vec<i64> = vec![];
    
    let mut maps :Vec<Vec<[i64; 3]>> = vec![];

    for line in reader.lines() {
        if let Ok(content) = line {
            // read seeds numbers
            if content.starts_with("seeds: ") {
                for num in re.find_iter(&content) {
                    seeds.push(num.as_str().parse::<i64>().unwrap());
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

    let mut values :Vec<i64> = vec![];

    for seed in seeds.iter() {
        let mut value :i64 = *seed;
        for map in maps.iter() {
            for entry in map.iter() {
                if (value >= entry[1]) && (value < entry[1] + entry[2]) {
                    value = entry[0] + value - entry[1];
                    break;
                }
            }
        }
        values.push(value);
    }

    result = *values.iter().min().unwrap();

    println!("Final answer: {}", result);

    Ok(())
}
