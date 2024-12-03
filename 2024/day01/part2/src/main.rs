use std::io::Read;
use std::fs::File;
use std::io::{self, BufReader, prelude::*};
use std::path::{absolute, Path};
use std::collections::HashMap;

fn main() {
    //print current working directory
    println!( "{}", std::env::current_dir().unwrap().display() );
    let mut file = File::open("../input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut vec1 = Vec::new();
    let mut map: HashMap<i32, i32> = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}", line);
        let mut parts = line.split(" ");
        let mut i = 0;
        for part in parts {
            if ! part.is_empty()  {
                if i == 0 {
                    vec1.push(part.parse::<i32>().unwrap());
                    i = i + 1;
                }
                else {
                    let mut key = part.parse::<i32>().unwrap();
                    match map.get(&key) {
                        Some( num ) => {
                            map.insert(key, num + 1);
                        },
                        None => {
                            map.insert(key, 1);
                        }
                    }
                }
            }
        }
    }

    let mut res = 0;

    for num in vec1 {
        match map.get(&num) {
            Some( count ) => {
                    res += num * count ;
            },
            None => {
            }
        }
    }


    println!("Sum: {}", res);
}
