use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("../input");
    let mut reader = BufReader::new(file.unwrap());

    for line in &mut reader.lines() {
        let mut line = line.unwrap();
        loop {
            match line.find("mul(") {
                Some(index) => {
                    line = line.split_off(index + 4);
                    match line.find(",") {
                        Some(index) => {
                            match  line[0..index].parse::<i32>() {
                                case Ok(first_number) => {
                                    line = line.split_off(index + 1);
                                    match line.find(")") {
                                        Some(index) => {
                                            let second_number = &line[0..index];
                                            let second_number = second_number.parse::<i32>().unwrap();
                                            println!("{} * {} = {}", first_number, second_number, first_number * second_number);
                                            break;
                                        }
                                        None => break,
                                    }
                                }
                            }

                            line = line.split_off(index + 1);
                            match line.find(")") {
                                Some(index) => {
                                    let second_number = &line[0..index];
                                    let first_number = first_number.parse::<i32>().unwrap();
                                    let second_number = second_number.parse::<i32>().unwrap();
                                    println!("{} * {} = {}", first_number, second_number, first_number * second_number);
                                    break;
                                }
                                None => break,
                            }
                        }
                        None => break,
                    }

                }
                None => break,
            }
        }
    }
}
