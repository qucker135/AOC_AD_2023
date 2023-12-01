use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let digits : Vec<String> = vec![
        "one".to_string(),
        "two".to_string(),
        "three".to_string(),
        "four".to_string(),
        "five".to_string(),
        "six".to_string(),
        "seven".to_string(),
        "eight".to_string(),
        "nine".to_string()
    ];

    let mut result = 0;

    for line in reader.lines() {
        if let Ok(content) = line {
            let mut i1 = content.find(|c: char| c.is_digit(10));
            let mut i2 = content.rfind(|c: char| c.is_digit(10));

            let mut d1 = 0;
            let mut d2 = 0;

            if let (Some(i1), Some(i2)) = (i1, i2) {
                d1 = content.chars().nth(i1).unwrap().to_digit(10).unwrap(); 
                d2 = content.chars().nth(i2).unwrap().to_digit(10).unwrap();
            }

            for (pos, e) in digits.iter().enumerate() {
                let (i1_tmp, i2_tmp) = (
                    content.find(e),
                    content.rfind(e)
                );
                
                if let (Some(i1_tmp), Some(i2_tmp)) = (i1_tmp, i2_tmp) {
                    if i1.is_none(){
                        i1 = Some(i1_tmp);
                        d1 = (pos + 1) as u32;
                    }
                    else if i1_tmp < i1.unwrap(){
                        i1 = Some(i1_tmp);
                        d1 = (pos + 1) as u32;
                    }

                    if i2.is_none(){
                        i2 = Some(i2_tmp);
                        d2 = (pos + 1) as u32;
                    }
                    else if i2_tmp > i2.unwrap(){
                        i2 = Some(i2_tmp);
                        d2 = (pos + 1) as u32;
                    }
                }
                
            } 

            result += d1 * 10 + d2;

        }
    }

    println!("Final answer: {}", result);

    Ok(())
}
