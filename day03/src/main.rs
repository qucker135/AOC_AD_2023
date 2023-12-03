use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut result = 0;

    let mut scheme :Vec<String> = vec![];

    for line in reader.lines() {
        if let Ok(content) = line {
            scheme.push(content);
        }
    }

    for (line_nr, _line) in scheme.iter().enumerate() {
        for (sym_nr, sym) in scheme[line_nr].chars().enumerate() {
            if sym == '*' {
                // println!("{}", sym_nr);
                let mut neigh_nums :Vec<i32> = vec![];
                
                // LEFT
                if sym_nr > 0 && scheme[line_nr].chars().nth(sym_nr - 1).unwrap().is_ascii_digit() {
                    let mut start = sym_nr - 1;
                    while start >= 1 && scheme[line_nr].chars().nth(start - 1).unwrap().is_ascii_digit() {
                        start -= 1;
                    }
                    neigh_nums.push((&scheme[line_nr][start..sym_nr]).parse::<i32>().unwrap());
                }

                // RIGHT
                if sym_nr + 1 < scheme[line_nr].len() && scheme[line_nr].chars().nth(sym_nr + 1).unwrap().is_ascii_digit() {
                    let mut end = sym_nr + 2;
                    while end < scheme[line_nr].len() && scheme[line_nr].chars().nth(end).unwrap().is_ascii_digit() {
                        end += 1;
                    }
                    neigh_nums.push((&scheme[line_nr][sym_nr+1..end]).parse::<i32>().unwrap());
                }

                // ABOVE
                // assume all lines have equal length
                if line_nr > 0 {
                    if scheme[line_nr - 1].chars().nth(sym_nr).unwrap().is_ascii_digit() {
                        //find start and end
                        let mut start = sym_nr;
                        let mut end = sym_nr + 1;

                        while start >= 1 && scheme[line_nr - 1].chars().nth(start - 1).unwrap().is_ascii_digit() {
                            start -= 1;
                        }

                        while end < scheme[line_nr - 1].len() && scheme[line_nr - 1].chars().nth(end).unwrap().is_ascii_digit() {
                            end += 1;
                        }

                        neigh_nums.push((&scheme[line_nr - 1][start..end]).parse::<i32>().unwrap());
                    }
                    else {
                        //algo similar to left & right cases, but for line above
                        if sym_nr > 0 && scheme[line_nr - 1].chars().nth(sym_nr - 1).unwrap().is_ascii_digit() {
                            let mut start = sym_nr - 1;
                            while start >= 1 && scheme[line_nr - 1].chars().nth(start - 1).unwrap().is_ascii_digit() {
                                start -= 1;
                            }
                            neigh_nums.push((&scheme[line_nr - 1][start..sym_nr]).parse::<i32>().unwrap());
                        }
                        
                        if sym_nr + 1 < scheme[line_nr - 1].len() && scheme[line_nr - 1].chars().nth(sym_nr + 1).unwrap().is_ascii_digit() {
                            let mut end = sym_nr + 2;
                            while end < scheme[line_nr - 1].len() && scheme[line_nr - 1].chars().nth(end).unwrap().is_ascii_digit() {
                                end += 1;
                            }
                            neigh_nums.push((&scheme[line_nr - 1][sym_nr+1..end]).parse::<i32>().unwrap());
                        }
                    }
                } 


                // BELOW
                // as above
                if line_nr + 1 < scheme.len() {
                     if scheme[line_nr + 1].chars().nth(sym_nr).unwrap().is_ascii_digit() {
                        //find start and end
                        let mut start = sym_nr;
                        let mut end = sym_nr + 1;

                        while start >= 1 && scheme[line_nr + 1].chars().nth(start - 1).unwrap().is_ascii_digit() {
                            start -= 1;
                        }

                        while end < scheme[line_nr + 1].len() && scheme[line_nr + 1].chars().nth(end).unwrap().is_ascii_digit() {
                            end += 1;
                        }

                        neigh_nums.push((&scheme[line_nr + 1][start..end]).parse::<i32>().unwrap());
                    }
                    else {
                        //algo similar to left & right cases, but for line below
                        if sym_nr > 0 && scheme[line_nr + 1].chars().nth(sym_nr - 1).unwrap().is_ascii_digit() {
                            let mut start = sym_nr - 1;
                            while start >= 1 && scheme[line_nr + 1].chars().nth(start - 1).unwrap().is_ascii_digit() {
                                start -= 1;
                            }
                            neigh_nums.push((&scheme[line_nr + 1][start..sym_nr]).parse::<i32>().unwrap());
                        }
                        
                        if sym_nr + 1 < scheme[line_nr + 1].len() && scheme[line_nr + 1].chars().nth(sym_nr + 1).unwrap().is_ascii_digit() {
                            let mut end = sym_nr + 2;
                            while end < scheme[line_nr + 1].len() && scheme[line_nr + 1].chars().nth(end).unwrap().is_ascii_digit() {
                                end += 1;
                            }
                            neigh_nums.push((&scheme[line_nr + 1][sym_nr+1..end]).parse::<i32>().unwrap());
                        }
                    }
                   
                }

                if neigh_nums.len() == 2 {
                    result += neigh_nums[0] * neigh_nums[1];
                }
            }
        }
    }

    println!("Final answer: {}", result);

    Ok(())
}
