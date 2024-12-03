use std::io::Read;
use std::fs::File;
use std::io::{self, BufReader, prelude::*};
use std::path::{absolute, Path};



fn main() {
    println!( "{}", std::env::current_dir().unwrap().display() );
    let mut file = File::open("../input").unwrap();
    let reader = BufReader::new(file);

    let mut safeReports = 0;
    'outer: for line in reader.lines() {
        let line = line.unwrap();
        let mut parts = line.split(" ");

        let mut diffVec : Vec<i8> = Vec::new();

        let mut first = true;
        let mut previous = 0;
        for part in parts {
            if first {
                previous = part.parse::<i8>().unwrap();
                first = false;
            }
            else {
                let current =  part.parse::<i8>().unwrap();
                let diff = current - previous;
                diffVec.push(diff);
                previous = current;
            }
        }

        let increasing = diffVec[0] > 0 ;
        for diff in diffVec {
            if increasing && diff < 1 || diff > 3 {
                continue 'outer;
            } else if !increasing && diff > -1 || diff < -3 {
                continue 'outer;
            }
        }
        safeReports += 1;
    }

    println!("Safe reports: {}", safeReports);
}
