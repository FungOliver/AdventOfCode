use std::fs::File;
use std::io::{BufRead, BufReader};


/*

     ---> x
     |
     |
     v y

 */



fn checkRight(dim: &Vec<Vec<char>>  , start : (usize, usize)) -> bool {
    let (x, y ) = start;
    if dim[0].len() -1 - x < 3 {
        return false;
    }
    dim[x][y] == 'X' && dim[x+1][y] == 'M' && dim[x+2][y] == 'A' && dim[x+3][y] == 'S'
}

fn checkLeft (dim: &Vec<Vec<char>>, start : (usize , usize)) -> bool {
    let (x, y ) = start;
    if x < 3 {
        return false;
    }
    dim[x][y] == 'X' && dim[x-1][y] == 'M' && dim[x-2][y] == 'A' && dim[x-3][y] == 'S'
}

fn checkUp (dim: &Vec<Vec<char>>, start : (usize , usize)) -> bool {
    let (x, y ) = start;
    if y < 3 {
        return false;
    }
    dim[x][y] == 'X' && dim[x][y-1] == 'M' && dim[x][y-2] == 'A' && dim[x][y-3] == 'S'
}

fn checkDown (dim: &Vec<Vec<char>>, start : (usize , usize)) -> bool {
    let (x, y ) = start;
    if dim.len() - y -1 < 3 {
        return false;
    }
    dim[x][y] == 'X' && dim[x][y+1] == 'M' && dim[x][y+2] == 'A' && dim[x][y+3] == 'S'
}

fn checkUpLeft (dim: &Vec<Vec<char>>, start : (usize , usize)) -> bool {
    let (x, y ) = start;
    if x < 3 || y < 3 {
        return false;
    }
    dim[x][y] == 'X' && dim[x-1][y-1] == 'M' && dim[x-2][y-2] == 'A' && dim[x-3][y-3] == 'S'
}

fn checkUpRight (dim: &Vec<Vec<char>>, start : (usize , usize)) -> bool {
    let (x, y ) = start;
    if x < 3 || dim[0].len() - 1 - y < 3 {
        return false;
    }
    dim[x][y] == 'X' && dim[x-1][y+1] == 'M' && dim[x-2][y+2] == 'A' && dim[x-3][y+3] == 'S'
}

fn checkDownLeft (dim: &Vec<Vec<char>>, start : (usize , usize)) -> bool {
    let (x, y ) = start;
    if dim.len() - 1 - x < 3 || y < 3 {
        return false;
    }
    dim[x][y] == 'X' && dim[x+1][y-1] == 'M' && dim[x+2][y-2] == 'A' && dim[x+3][y-3] == 'S'
}

fn checkDownRight (dim: &Vec<Vec<char>>, start : (usize , usize)) -> bool {
    let (x, y ) = start;
    if dim.len() - 1 - x < 3 || dim[0].len() - 1 - y < 3 {
        return false;
    }
    dim[x][y] == 'X' && dim[x+1][y+1] == 'M' && dim[x+2][y+2] == 'A' && dim[x+3][y+3] == 'S'
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
    for i in 0..dim.len() {
        for j in 0..dim[i].len() {
            xmasCount += checkRight(&dim, (i, j)) as i32;
            xmasCount += checkLeft(&dim, (i, j)) as i32;
            xmasCount += checkUp(&dim, (i, j)) as i32;
            xmasCount += checkDown(&dim, (i, j)) as i32;
            xmasCount += checkUpLeft(&dim, (i, j)) as i32;
            xmasCount += checkUpRight(&dim, (i, j)) as i32;
            xmasCount += checkDownLeft(&dim, (i, j)) as i32;
            xmasCount += checkDownRight(&dim, (i, j)) as i32;
        }
    }
    println!("{}", xmasCount);
}