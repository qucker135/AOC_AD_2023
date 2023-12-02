use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut result = 0;

    for line in reader.lines() {
        if let Ok(content) = line {
            let game_splitted : Vec<&str> = content.split(": ").collect();

            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for trial in game_splitted[1].split("; ") {
                for cube in trial.split(", ") {
                    if cube.ends_with(" red") { 
                        let new_red = (&cube[..cube.len()-4]).parse::<i32>().unwrap();
                        if new_red > red {
                            red = new_red;
                        }
                    }
                    if cube.ends_with(" green") {
                        let new_green = (&cube[..cube.len()-6]).parse::<i32>().unwrap();
                        if new_green > green {
                            green = new_green;
                        }
                    }
                    if cube.ends_with(" blue") {
                        let new_blue = (&cube[..cube.len()-5]).parse::<i32>().unwrap();
                        if new_blue > blue {
                            blue = new_blue;
                        }
                    }
                }
            }

            result += red * green * blue;
        }
    }

    println!("Final answer: {}", result);

    Ok(())
}
