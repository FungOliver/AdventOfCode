use std::io::Read;
use std::fs::File;
use std::io::{self, BufReader, prelude::*};
use std::path::{absolute, Path};

fn main() {
    //print current working directory
    println!( "{}", std::env::current_dir().unwrap().display() );
    let mut file = File::open("../input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

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
                    vec2.push(part.parse::<i32>().unwrap());
                }
            }
        }
    }

    vec1.sort();
    vec2.sort();

    let mut sum = 0;
    for i in 0..vec1.len(){
         sum += ( vec1[i] - vec2[i]).abs();
    }

    println!("Sum: {}", sum);
}
