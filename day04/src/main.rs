use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;
use regex::Regex;

fn push_or_inc(v: &mut Vec<i32>, ind: usize, inc: i32){
    if ind >= v.len() {
        v.push(inc);
    }
    else {
        v[ind] += inc;
    }
}

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut result = 0;

    let re = Regex::new(r"[0-9]+").unwrap();

    let mut scratchcards_nrs :Vec<i32> = vec![];

    for (line_nr, line) in reader.lines().enumerate() {
        if let Ok(content) = line {
            push_or_inc(&mut scratchcards_nrs, line_nr, 1); 

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

            let bfr = scratchcards_nrs[line_nr];

            for i in 1..length + 1 {
                push_or_inc(&mut scratchcards_nrs, line_nr + i, bfr);
            }
        }
    }

    for v in scratchcards_nrs.iter() {
        result += v;
    }

    println!("Final answer: {}", result);

    Ok(())
}
