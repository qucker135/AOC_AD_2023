use regex::Regex;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut result = 0;

    let re = Regex::new(r"-?[0-9]+").unwrap();

    for line in reader.lines().map_while(Result::ok) {
        let mut diffs: Vec<Vec<i64>> = vec![];

        diffs.push(vec![]);

        for m in re.find_iter(&line) {
            let parsed = m.as_str().parse::<i64>().unwrap();

            diffs[0].push(parsed);
        }

        while diffs.last().unwrap().len() > 1 && diffs.last().unwrap().iter().any(|&x| x != 0) {
            let mut partial_diffs: Vec<i64> = vec![];

            let last_diffs = diffs.last().unwrap();

            for i in 0..last_diffs.len() - 1 {
                partial_diffs.push(last_diffs[i + 1] - last_diffs[i]);
            }

            diffs.push(partial_diffs);
        }

        let mut extrapolated_value: i64 = 0;
        for partial_diffs in diffs.iter().rev() {
            extrapolated_value = partial_diffs[0] - extrapolated_value;
        }

        result += extrapolated_value;
    }

    println!("Final answer: {}", result);

    Ok(())
}
