use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;
use std::cmp::Ordering;
use std::iter::zip;

fn compare_hands(h1: (String, i32), h2: (String, i32)) -> std::cmp::Ordering {
    if h1.0 == h2.0 {
        return Ordering::Equal;
    }
    else {
        let mut h1_to_occs :HashMap<char, i32> = HashMap::new();
        let mut h2_to_occs :HashMap<char, i32> = HashMap::new();

        for (c1, c2) in zip(h1.0.chars(), h2.0.chars()) {
            h1_to_occs.entry(c1).and_modify(|counter| *counter += 1).or_insert(1);
            h2_to_occs.entry(c2).and_modify(|counter| *counter += 1).or_insert(1);
        }
        
        let j1 :Option<i32> = h1_to_occs.remove(&'J');

        if j1.is_some() {
            if h1_to_occs.is_empty() {
                h1_to_occs.insert('A', j1.unwrap());
            }
            else {
                let key_with_max_value_h1 = h1_to_occs.iter().max_by_key(|entry| entry.1).unwrap();

                h1_to_occs.insert(*key_with_max_value_h1.0, h1_to_occs[key_with_max_value_h1.0] + j1.unwrap());
            }
        }

        let j2 :Option<i32> = h2_to_occs.remove(&'J');

        if j2.is_some() {
            if h2_to_occs.is_empty() {
                h2_to_occs.insert('A', j2.unwrap());
            }
            else {
                let key_with_max_value_h2 = h2_to_occs.iter().max_by_key(|entry| entry.1).unwrap();

                h2_to_occs.insert(*key_with_max_value_h2.0, h2_to_occs[key_with_max_value_h2.0] + j2.unwrap());
            }
        }
        
        let mut occs1 :Vec<i32> = h1_to_occs.into_values().collect();
        let mut occs2 :Vec<i32> = h2_to_occs.into_values().collect();

        occs1.sort_unstable();
        occs2.sort_unstable();
        if occs1.len() < occs2.len() || occs1.last() > occs2.last() {
            return Ordering::Greater;
        }
        else if occs1 == occs2 {
            let cards_by_strength :[char; 13] = ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'];
            for (c1, c2) in zip(h1.0.chars(), h2.0.chars()) {
                let c1_index = cards_by_strength.iter().position(|&c| c == c1);
                let c2_index = cards_by_strength.iter().position(|&c| c == c2);
                if c1_index < c2_index {
                    return Ordering::Greater;
                }
                else if c1_index > c2_index {
                    return Ordering::Less;
                }
            }
        }
        return Ordering::Less;
    }
}

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut result = 0;

    let mut hands_data :Vec<(String, i32)> = vec![];

    for line in reader.lines() {
        if let Ok(content) = line {
            let splitted :Vec<&str> = content.split(" ").collect();

            let bid = splitted[1].parse::<i32>().unwrap();

            hands_data.push((splitted[0].to_string(), bid));
        }
    }

    hands_data.sort_by(|h1, h2| compare_hands(h1.clone(), h2.clone()));

    for (h_nr, hand) in hands_data.iter().enumerate() {
        result += ((h_nr + 1) as i32) * hand.1;
    }

    println!("Final answer: {}", result);

    Ok(())
}
