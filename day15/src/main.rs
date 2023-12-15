#![feature(extract_if)]
use std::collections::LinkedList;
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

    let mut map: Vec<LinkedList<(String, i32)>> = vec![];

    for _i in 0..256 {
        map.push(LinkedList::new());
    }

    let mut sequence: String = String::from("");

    for line in reader.lines().map_while(Result::ok) {
        sequence.push_str(line.trim());
    }

    for com in sequence.split(',') {
        if com.ends_with('-') {
            let label: String = com[0..com.len() - 1].to_owned();
            let box_nr = hash(&label) as usize;

            map[box_nr].extract_if(|(l, _f)| *l == label).for_each(drop);
        } else if com.chars().last().is_some() {
            let splitted: Vec<&str> = com.split('=').collect();
            let box_nr = hash(splitted[0]) as usize;
            let focal: i32 = splitted[1].parse::<i32>().unwrap();

            if map[box_nr].iter().any(|(l, _f)| *l == splitted[0]) {
                map[box_nr] = map[box_nr]
                    .iter()
                    .map(|(l, f)| {
                        if l.eq(&splitted[0]) {
                            (l.clone(), focal)
                        } else {
                            (l.clone(), *f)
                        }
                    })
                    .collect();
            } else {
                map[box_nr].push_back((splitted[0].to_owned(), focal));
            }
        }
    }

    for (i_b, b) in map.iter().enumerate() {
        for (i_r, r) in b.iter().enumerate() {
            result += ((i_b + 1) * (i_r + 1) * r.1 as usize) as i64;
        }
    }

    println!("Final answer: {}", result);

    Ok(())
}
