use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut instructions: String = "".to_string();
    let mut map: HashMap<String, (String, String)> = HashMap::new();

    for (line_nr, line) in reader.lines().enumerate() {
        if let Ok(content) = line {
            if line_nr == 0 {
                instructions = content.to_string();
            } else if line_nr > 1 {
                let key: String = content[0..3].to_string();
                let v1: String = content[7..10].to_string();
                let v2: String = content[12..15].to_string();

                map.insert(key, (v1, v2));
            }
        }
    }

    let mut steps_for_pos: Vec<i64> = vec![];

    for pos in map.keys() {
        if pos.chars().nth(2) == Some('A') {
            let mut steps = 0;

            let mut cur_pos = pos.clone();

            while cur_pos.chars().nth(2) != Some('Z') {
                let instr_len = instructions.len();

                if instructions.chars().nth(steps % instr_len) == Some('L') {
                    cur_pos = map[&cur_pos].0.clone();
                } else {
                    cur_pos = map[&cur_pos].1.clone();
                }

                steps += 1;
            }

            steps_for_pos.push(steps as i64);
        }
    }

    let result = steps_for_pos.into_iter().reduce(lcm).unwrap();

    println!("Final answer: {}", result);

    Ok(())
}
