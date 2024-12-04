use std::fs::File;
use std::io::{BufRead, BufReader};


fn checkMas(dim: &Vec<Vec<char>>, start: (usize, usize)) -> bool {
    let (x, y) = start;
    if dim[x][y] != 'A' {
        return false;
    }

    let diag1 = (dim[x - 1][y - 1] == 'M' && dim[x + 1][y + 1] == 'S') || (dim[x - 1][y - 1] == 'S' && dim[x + 1][y + 1] == 'M');
    if (!diag1 ){
        return  false;
    }

    (dim[x - 1][y + 1] == 'M' && dim[x + 1][y - 1] == 'S') || (dim[x - 1][y + 1] == 'S' && dim[x + 1][y - 1] == 'M')
}

pub(crate) fn run() {
    let file = File::open("input");
    let mut reader = BufReader::new(file.unwrap());

    let mut dim: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let chars: Vec<char> = line.chars().collect();
        dim.push(chars)
    }

    let mut xmasCount = 0;
    for i in 1..dim.len() - 1 {
        for j in 1..dim[i].len() - 1 {
            xmasCount += checkMas(&dim, (i, j)) as i32;
        }
    }
    println!("{}", xmasCount);
}

// 4510
// 710