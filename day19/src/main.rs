use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Condition {
    Final {
        jump: String,
    },
    GreaterThan {
        attr: String,
        value: i64,
        jump: String,
    },
    LowerThan {
        attr: String,
        value: i64,
        jump: String,
    },
}

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut result: i64 = 0;

    let mut tmp_flag = true;

    let mut workflows: HashMap<String, Vec<Condition>> = HashMap::new();
    let mut parts: Vec<HashMap<String, i64>> = vec![];

    for line in reader.lines().map_while(Result::ok) {
        if line.is_empty() {
            tmp_flag = false;
        } else if tmp_flag {
            // read workflows
            let split_name: Vec<&str> = line[..line.len() - 1].split('{').collect();

            let key = split_name[0].to_string();
            let raw_conditions: Vec<&str> = split_name[1].split(',').collect();

            let value: Vec<Condition> = raw_conditions
                .into_iter()
                .map(|s: &str| {
                    if !s.contains(':') {
                        Condition::Final {
                            jump: s.to_string(),
                        }
                    } else {
                        let split_jump: Vec<&str> = s.split(':').collect();
                        if split_jump[0].contains('>') {
                            let split_comp: Vec<&str> = split_jump[0].split('>').collect();
                            let attr = split_comp[0].to_string();
                            let value = split_comp[1].parse::<i64>().unwrap();
                            let jump = split_jump[1].to_string();
                            Condition::GreaterThan { attr, value, jump }
                        } else {
                            let split_comp: Vec<&str> = split_jump[0].split('<').collect();
                            let attr = split_comp[0].to_string();
                            let value = split_comp[1].parse::<i64>().unwrap();
                            let jump = split_jump[1].to_string();
                            Condition::LowerThan { attr, value, jump }
                        }
                    }
                })
                .collect();

            workflows.insert(key, value);
        } else {
            // read parts
            parts.push(
                line[1..line.len() - 1]
                    .split(',')
                    .map(|s: &str| {
                        let tmp_split: Vec<_> = s.split('=').collect();
                        let key = tmp_split[0].to_string();
                        let value = tmp_split[1].parse::<i64>().unwrap();
                        (key, value)
                    })
                    .collect::<HashMap<_, _>>(),
            );
        }
    }

    for part in parts.iter() {
        let mut wf_name = String::from("in");
        let mut cond_id = 0usize;

        while !["A".to_string(), "R".to_string()].contains(&wf_name) {
            match &workflows.get(&wf_name).unwrap()[cond_id] {
                Condition::Final { jump } => {
                    wf_name = jump.clone();
                    cond_id = 0usize;
                }
                Condition::GreaterThan { attr, value, jump } => {
                    if part.get(&attr.clone()).unwrap() > value {
                        wf_name = jump.clone();
                        cond_id = 0usize;
                    } else {
                        cond_id += 1;
                    }
                }
                Condition::LowerThan { attr, value, jump } => {
                    if part.get(&attr.clone()).unwrap() < value {
                        wf_name = jump.clone();
                        cond_id = 0usize;
                    } else {
                        cond_id += 1;
                    }
                }
            }
        }

        if wf_name == *"A" {
            result += part.values().sum::<i64>();
        }
    }

    println!("Final answer: {:?}", result);

    Ok(())
}
