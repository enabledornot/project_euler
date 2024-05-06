use std::fs::File;
use std::io::{BufRead, BufReader};

fn make_step(begin: &mut [i32; 2], step: [i32; 2]) {
    begin[0] = begin[0] + step[0];
    begin[1] = begin[1] + step[1];
}

fn get_position(grid: &Vec<Vec<u32>>, poss: [i32;2]) -> Option<u32> {
    if poss[0] < 0 || poss[1] < 0 {
        return None;
    }
    if let Some(row) = grid.get(poss[0] as usize) {
        let e: Option<u32> = row.get(poss[1] as usize).copied();
        return e;
    }
    return None;
}

fn get_max_along_shift(grid: &Vec<Vec<u32>>, start: [i32; 2], step: [i32; 2]) -> u32 {
    let mut current = start;
    let mut current_product = 1;
    let mut current_max_product = 0;
    let mut prev_digits: Vec<u32> = Vec::new();
    while let Some(digit) = get_position(grid, current) {
        if digit == 0 {
            current_product = 1;
            prev_digits = Vec::new();
        }
        else {
            prev_digits.push(digit);
            current_product = current_product * digit;
            if prev_digits.len() == 5 {
                let last = prev_digits.remove(0);
                current_product = current_product / last;
            }
            if prev_digits.len() == 4 && current_max_product < current_product {
                current_max_product = current_product;
            }
        }
        make_step(&mut current, step);
    }

    return current_max_product;
}

fn get_max(grid: &Vec<Vec<u32>>) -> u32 {
    let len = grid.len() as i32;
    // horizontal
    let mut max: u32 = 0;
    for i in 0..len {
        let p = get_max_along_shift(&grid, [i,0], [0,1]);
        if p > max {
            max = p;
        }
    }
    // vertical
    for i in 0..len {
        let p = get_max_along_shift(&grid, [0,i], [1,0]);
        if p > max {
            max = p;
        }
    }
    // desc diagonal
    for i in 0..len {
        let p = get_max_along_shift(&grid, [i,0], [1,1]);
        if p > max {
            max = p;
        }
    }
    for i in 1..len {
        let p = get_max_along_shift(&grid, [0,i], [1,1]);
        if p > max {
            max = p;
        }
    }
    // asc diagonal
    for i in 0..len {
        let p = get_max_along_shift(&grid, [len-1,i],[-1,1]);
        if p > max {
            max = p;
        }
    }
    for i in 1..len {
        let p = get_max_along_shift(&grid, [i,0],[-1,1]);
        if p > max {
            max = p;
        }
    }
    return max;
}

fn main() -> Result<(), std::io::Error>{
    let file = File::open("input.txt")?;
    let read = BufReader::new(file);
    let mut grid: Vec<Vec<u32>> = Vec::new();
    for str in read.lines() {
        let line = str?;
        let mut row: Vec<u32> = Vec::new();
        let parts: Vec<&str> = line.split(" ").collect();
        for num in parts {
            row.push(num.parse().unwrap());
        }
        grid.push(row);
    }
    println!("The grid is {:?}",grid);
    let max = get_max(&grid);
    println!("The max is {:?}",max);
    Ok(())
}
