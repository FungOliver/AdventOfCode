use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("../input");
    let mut reader = BufReader::new(file.unwrap());

    let mut enabled = true;
    let mut sum = 0;
    for line in &mut reader.lines() {
        let mut line = line.unwrap();
        loop {
            if enabled {
                match (line.find ("don't()"), line.find("mul(")) {
                    (None, None) => break,
                    (Some(index), None) => {
                        println!("line = {}", line);
                        line = line.split_off(index + 7);
                        enabled = false;
                    }
                    (None, Some(index)) => {
                        line = line.split_off(index + 4);
                    }
                    (Some(index1), Some(index2)) => {
                        if index1 < index2 {
                            line = line.split_off(index1 + 7);
                            enabled = false;
                        } else {
                            line = line.split_off(index2 + 4);
                        }
                    }
                }

                let mut left_number = 0;
                match line.find(",") {
                    Some(index) => {
                        let str_ref = &line[..index];
                        match str_ref.parse::<i32>() {
                            Ok(num) => {
                                left_number = num;
                            }
                            Err(_) => {
                                println!("err 1 = {}", line);
                                continue;
                            }
                        }
                        line = line.split_off(index + 1);
                    }
                    None => continue,
                }

                // println!("line = {}", line);
                let mut right_number = 0;
                match line.find(")") {
                    Some(index) => {
                        let str_ref = &line[..index];
                        match str_ref.parse::<i32>() {
                            Ok(num) => {
                                right_number = num;
                            }
                            Err(_) => {
                                continue;
                            }
                        }
                        line = line.split_off(index + 1);
                    }
                    None => continue,
                }


                println!("{} * {} = {}", left_number, right_number, left_number * right_number);
                sum += left_number * right_number;
                // println!("{}", line)
            } else {
                match line.find("do()") {
                    Some(index) => {
                        line = line.split_off(index + 4);
                        enabled = true;
                    }
                    None => break,
                }
            }
        }
    }
    println!("sum = {}", sum);
}