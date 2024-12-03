use std::io::Read;
use std::fs::File;
use std::io::{self, BufReader, prelude::*};
use std::path::{absolute, Path};


fn main() {
    println!("{}", std::env::current_dir().unwrap().display());
    let mut file = File::open("../input").unwrap();
    // let mut file = File::open("../test").unwrap();
    let reader = BufReader::new(file);

    let mut safeReports = 0;
    'outer: for line in reader.lines() {
        let line = line.unwrap();
        let mut parts = line.split(" ");

        let mut diffVec: Vec<i8> = Vec::new();

        let mut first = true;
        let mut previous = 0;
        for part in parts {
            if first {
                previous = part.parse::<i8>().unwrap();
                first = false;
            } else {
                let current = part.parse::<i8>().unwrap();
                let diff = current - previous;
                diffVec.push(diff);
                previous = current;
            }
        }

        let increasing: bool;

        let numberIncreasing = diffVec.iter().filter(|&x| *x > 0).count();
        let numberDecreasing = diffVec.iter().filter(|&x| *x < 0).count();

        if numberDecreasing == numberIncreasing {
            // println!("equal {}", line);
            continue 'outer;
        }

        increasing = numberIncreasing > numberDecreasing;

        let mut dampened = false;

        let mut i = 0;
        println!("{:?} , {}", diffVec, increasing);
        println!("{}", line);
        while i < diffVec.len() {
            let diff = diffVec[i];
            if increasing && (diff < 1 || diff > 3) {
                if !dampened {
                    dampened = true;
                    if (i != diffVec.len() - 1 ) {
                        if (diff + diffVec[i + 1] >= 1 && diff + diffVec[i + 1] <= 3) {
                            i = i + 1;
                        } else if i != 0 && diff + diffVec[i - 1] >= 1 && diff + diffVec[i - 1] <= 3 {
                            ;
                        } else if (i == 0 ) {
                            ;
                        } else {
                            println!("unsafe {}", line);
                            continue 'outer;
                        }
                    }
                } else {
                    println!("unsafe {}", line);
                    continue 'outer;
                }
            } else if !increasing && (diff > -1 || diff < -3) {
                if !dampened {
                    dampened = true;
                    if (i != diffVec.len() - 1 ) {
                        if (diff + diffVec[i + 1] <= -1 && diff + diffVec[i + 1] >= -3) {
                            i = i + 1;
                        } else if i != 0 && diff + diffVec[i - 1] <= -1 && diff + diffVec[i - 1] >= -3 {
                            ;
                        } else if( i == 0 ) {
                            ;
                        } else{
                            println!("unsafe {}", line);
                            continue 'outer;
                        }
                    }
                } else {
                    println!("unsafe {}", line);
                    continue 'outer;
                }
            }
            i = i + 1;
        }
        safeReports += 1;
        println!("safe {}", line);
        // println!("{:?} , {}" ,diffVec, increasing);
    }
    println!("Safe reports: {}", safeReports);
}
