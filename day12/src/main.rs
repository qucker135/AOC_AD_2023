use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn compute_matches(
    pattern: &str,
    nums: &[i32],
    map: &mut HashMap<(usize, usize, i32), i64>,
    p_index: usize,
    n_index: usize,
) -> i64 {
    let mut left: i32 = 0;
    let mut tmp_ind: usize = n_index;
    while tmp_ind > 0 && pattern.chars().nth(tmp_ind - 1) == Some('#') {
        tmp_ind -= 1;
        left += 1;
    }
    if n_index >= pattern.len() && p_index >= nums.len() {
        map.insert((p_index, n_index, left), 1);
        1
    } else if n_index >= pattern.len() {
        map.insert((p_index, n_index, left), 0);
        0
    } else if map.contains_key(&(p_index, n_index, left)) {
        return map[&(p_index, n_index, left)];
    } else if pattern.chars().nth(n_index) == Some('.') {
        // non-terminating dot
        if left == 0 {
            let sols = compute_matches(pattern, nums, map, p_index, n_index + 1);
            map.insert((p_index, n_index, left), sols);
            return sols;
        }
        // terminating dot
        else if left != nums[p_index] {
            map.insert((p_index, n_index, left), 0);
            return 0;
        } else {
            let sols = compute_matches(pattern, nums, map, p_index + 1, n_index + 1);
            map.insert((p_index, n_index, left), sols);
            return sols;
        }
    } else if pattern.chars().nth(n_index) == Some('#') {
        if p_index >= nums.len() || left + 1 > nums[p_index] {
            map.insert((p_index, n_index, left), 0);
            return 0;
        } else {
            let sols = compute_matches(pattern, nums, map, p_index, n_index + 1);
            map.insert((p_index, n_index, left), sols);
            return sols;
        }
    }
    // question mark
    else {
        let pattern_dot = pattern.replacen('?', ".", 1);
        let sols_dot = if left == 0 {
            compute_matches(&pattern_dot, nums, map, p_index, n_index + 1)
        } else if left == nums[p_index] {
            compute_matches(&pattern_dot, nums, map, p_index + 1, n_index + 1)
        } else {
            0
        };
        let pattern_hash = pattern.replacen('?', "#", 1);
        let sols_hash = if p_index >= nums.len() || left + 1 > nums[p_index] {
            0
        } else {
            compute_matches(&pattern_hash, nums, map, p_index, n_index + 1)
        };
        map.insert((p_index, n_index, left), sols_dot + sols_hash);
        return sols_dot + sols_hash;
    }
}

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut result: i64 = 0;

    for line in reader.lines().map_while(Result::ok) {
        let mut map: HashMap<(usize, usize, i32), i64> = HashMap::new();
        // key: (usize, usize, i32) -> (p_index, n_index, left)
        // value: i64 - number of partial solutions

        let splitted: Vec<&str> = line.split(' ').collect();

        let mut pattern_long: String = [
            splitted[0].to_string(),
            splitted[0].to_string(),
            splitted[0].to_string(),
            splitted[0].to_string(),
            splitted[0].to_string(),
        ]
        .join("?");

        pattern_long.push('.');

        let nums: Vec<i32> = splitted[1]
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        let nums_long: Vec<i32> = nums.iter().cycle().take(nums.len() * 5).copied().collect();

        result += compute_matches(&pattern_long, &nums_long, &mut map, 0, 0);
    }

    println!("Final answer: {}", result);

    Ok(())
}
