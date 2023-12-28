use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

const MINIMUM_VALUE: i64 = 1;
const MAXIMUM_VALUE: i64 = 4000;

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
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

impl Condition {
    fn get_jump(&self) -> &str {
        match self {
            Condition::Final { jump } => jump,
            Condition::GreaterThan {
                attr: _,
                value: _,
                jump,
            } => jump,
            Condition::LowerThan {
                attr: _,
                value: _,
                jump,
            } => jump,
        }
    }
    fn negation(&self) -> Option<Condition> {
        match self {
            Condition::Final { jump: _ } => None,
            Condition::GreaterThan { attr, value, jump } => Some(Condition::LowerThan {
                attr: attr.to_string(),
                value: *value + 1,
                jump: jump.to_string(),
            }),
            Condition::LowerThan { attr, value, jump } => Some(Condition::GreaterThan {
                attr: attr.to_string(),
                value: *value - 1,
                jump: jump.to_string(),
            }),
        }
    }
}

fn evaluate_conds(node_conds: &[Condition]) -> i64 {
    let (mut min_x, mut max_x) = (MINIMUM_VALUE, MAXIMUM_VALUE);
    let (mut min_m, mut max_m) = (MINIMUM_VALUE, MAXIMUM_VALUE);
    let (mut min_a, mut max_a) = (MINIMUM_VALUE, MAXIMUM_VALUE);
    let (mut min_s, mut max_s) = (MINIMUM_VALUE, MAXIMUM_VALUE);

    for cond in node_conds.iter() {
        match cond {
            Condition::GreaterThan {
                attr,
                value,
                jump: _,
            } => {
                if attr == "x" {
                    min_x = max(min_x, *value + 1);
                } else if attr == "m" {
                    min_m = max(min_m, *value + 1);
                } else if attr == "a" {
                    min_a = max(min_a, *value + 1);
                } else if attr == "s" {
                    min_s = max(min_s, *value + 1);
                }
            }
            Condition::LowerThan {
                attr,
                value,
                jump: _,
            } => {
                if attr == "x" {
                    max_x = min(max_x, *value - 1);
                } else if attr == "m" {
                    max_m = min(max_m, *value - 1);
                } else if attr == "a" {
                    max_a = min(max_a, *value - 1);
                } else if attr == "s" {
                    max_s = min(max_s, *value - 1);
                }
            }
            _ => {}
        }
    }

    max(0, max_x - min_x + 1)
        * max(0, max_m - min_m + 1)
        * max(0, max_a - min_a + 1)
        * max(0, max_s - min_s + 1)
}

fn compute_combinations(
    node_conds: &mut Vec<Condition>,
    workflow_it: &mut Vec<Vec<Condition>>,
    workflows: &HashMap<String, Vec<Condition>>,
    res: &mut i64,
) {
    if !workflow_it.is_empty() {
        let last_workflow = workflow_it.last().unwrap().clone();

        for cond in last_workflow.iter() {
            node_conds.push(cond.clone());

            if cond.get_jump() == "A" {
                *res += evaluate_conds(node_conds);
            } else if cond.get_jump() != "R" {
                workflow_it.push(workflows[cond.get_jump()].clone());
                compute_combinations(node_conds, workflow_it, workflows, res);
                workflow_it.pop();
            }

            node_conds.pop();
            if let Some(neg_cond) = cond.negation() {
                node_conds.push(neg_cond);
            }
        }

        for _ in 0..last_workflow.len() - 1 {
            node_conds.pop();
        }
    }
}

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut result: i64 = 0;

    let mut tmp_flag = true;

    let mut workflows: HashMap<String, Vec<Condition>> = HashMap::new();

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
        }
    }

    let mut node_conds: Vec<Condition> = vec![];

    let mut workflow_it: Vec<Vec<Condition>> = vec![workflows["in"].clone()];

    compute_combinations(&mut node_conds, &mut workflow_it, &workflows, &mut result);

    println!("Final answer: {:?}", result);

    Ok(())
}
