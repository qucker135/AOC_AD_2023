use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut result = 0;

    let mut instructions :String = "".to_string();
    let mut map :HashMap<String, (String, String)> = HashMap::new();

    for (line_nr, line) in reader.lines().enumerate() {
        if let Ok(content) = line {
            if line_nr == 0 {
                instructions = content.to_string();
            }
            else if line_nr > 1 {
                let key :String = content[0..3].to_string();
                let v1 :String = content[7..10].to_string();
                let v2 :String = content[12..15].to_string();

                map.insert(key, (v1, v2));
            }
        }
    }

    let mut cur_pos = "AAA".to_string();

    while cur_pos != "ZZZ".to_string() {
        let instr_len = instructions.len();

        if instructions.chars().nth(result % instr_len) == Some('L') {
            cur_pos = map[&cur_pos].0.clone();
        }

        else {
            cur_pos = map[&cur_pos].1.clone();
        }

        result += 1;
    }

    println!("Final answer: {}", result);

    Ok(())
}
