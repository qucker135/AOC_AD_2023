use std::collections::{HashMap, LinkedList};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug, Clone)]
struct Module {
    module_type: ModuleType,
    destinations: Vec<String>,
}

#[derive(Debug, Clone)]
enum ModuleType {
    FF { state: bool },
    Conj { inputs: HashMap<String, bool> },
    Broadcaster,
    Button,
}

impl Module {
    fn new(module_type: ModuleType, destinations: Vec<String>) -> Module {
        Module {
            module_type,
            destinations,
        }
    }
}

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut modules: HashMap<String, Module> = HashMap::new();

    modules.insert(
        "button".to_string(),
        Module::new(ModuleType::Button, vec!["broadcaster".to_string()]),
    );

    for line in reader.lines().map_while(Result::ok) {
        let split_arrow: Vec<&str> = line.split(" -> ").collect();
        let destinations: Vec<String> = split_arrow[1].split(", ").map(|s| s.to_string()).collect();
        if split_arrow[0] == "broadcaster" {
            modules.insert(
                split_arrow[0].to_string(),
                Module::new(ModuleType::Broadcaster, destinations),
            );
        } else if split_arrow[0].starts_with('%') {
            modules.insert(
                split_arrow[0][1..].to_string(),
                Module::new(ModuleType::FF { state: false }, destinations),
            );
        } else if split_arrow[0].starts_with('&') {
            modules.insert(
                split_arrow[0][1..].to_string(),
                Module::new(
                    ModuleType::Conj {
                        inputs: HashMap::new(),
                    },
                    destinations,
                ),
            );
        }
    }

    for (name, module) in &modules.clone() {
        for destination in &module.destinations {
            if let Some(Module {
                module_type: ModuleType::Conj { inputs },
                ..
            }) = modules.get_mut(destination)
            {
                inputs.insert(name.to_string(), false);
            }
        }
    }

    // queue of signals
    let mut signals: LinkedList<(String, bool)> = LinkedList::new();

    let mut low_impulses: Vec<i64> = vec![];
    let mut high_impulses: Vec<i64> = vec![];

    for _ in 0..1000 {
        signals.push_back(("button".to_string(), false));

        // process signals, counting low and high impulses
        let mut low_impulse: i64 = 0;
        let mut high_impulse: i64 = 0;

        while let Some((name, signal)) = signals.pop_front() {
            for destination in &modules[&name].destinations.clone() {
                if signal {
                    high_impulse += 1;
                } else {
                    low_impulse += 1;
                }
                match modules.get_mut(destination) {
                    Some(Module {
                        module_type: ModuleType::Broadcaster,
                        ..
                    }) => {
                        signals.push_back((destination.to_string(), signal));
                    }
                    Some(Module {
                        module_type: ModuleType::FF { state },
                        ..
                    }) => {
                        if !signal {
                            *state = !*state;
                            signals.push_back((destination.to_string(), *state));
                        }
                    }
                    Some(Module {
                        module_type: ModuleType::Conj { inputs },
                        ..
                    }) => {
                        inputs.insert(name.clone(), signal);
                        signals
                            .push_back((destination.to_string(), !(inputs.values().all(|&x| x))));
                    }
                    _ => {}
                }
            }
        }

        low_impulses.push(low_impulse);
        high_impulses.push(high_impulse);
    }

    let result: i64 = low_impulses.iter().sum::<i64>() * high_impulses.iter().sum::<i64>();

    println!("Final answer: {:?}", result);

    Ok(())
}
