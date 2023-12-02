use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut result = 0;

    for line in reader.lines() {
        if let Ok(content) = line {
            let game_splitted : Vec<&str> = content.split(": ").collect();

            let index : i32 = (&game_splitted[0][5..]).parse::<i32>().unwrap();

            let mut flag = true;

            for trial in game_splitted[1].split("; ") {
                for cube in trial.split(", ") {
                    if cube.ends_with(" red") && (&cube[..cube.len()-4]).parse::<i32>().unwrap() > 12 {
                        flag = false;
                    }
                    if cube.ends_with(" green") && (&cube[..cube.len()-6]).parse::<i32>().unwrap() > 13 {
                        flag = false;
                    }
                    if cube.ends_with(" blue") && (&cube[..cube.len()-5]).parse::<i32>().unwrap() > 14 {
                        flag = false;
                    }
                }
            }

            if flag {
                result += index;
            }
        }
    }

    println!("Final answer: {}", result);

    Ok(())
}
