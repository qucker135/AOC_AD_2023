use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn subst(line: &str, i: usize) -> String {
    let mut res = line.to_string();

    let mut it = i;

    while let Some(ind) = res.find('?') {
        if it % 2 == 0 {
            res.replace_range(ind..ind + 1, ".");
        } else {
            res.replace_range(ind..ind + 1, "#");
        }
        it /= 2;
    }

    res
}

fn split_to_vec(s_line: String) -> Vec<i32> {
    s_line
        .split('.')
        .map(|s| s.len() as i32)
        .filter(|&x| x != 0)
        .collect()
}

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut result: i64 = 0;

    for line in reader.lines().map_while(Result::ok) {
        let splitted: Vec<&str> = line.split(' ').collect();

        let nums: Vec<i32> = splitted[1]
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        let occs = splitted[0].matches('?').count();

        let powered = 2usize.pow(occs as u32);

        for i in 0..powered {
            let subst_line = subst(splitted[0], i);

            if split_to_vec(subst_line) == nums {
                result += 1;
            }
        }
    }

    println!("Final answer: {}", result);

    Ok(())
}
