use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error>{
    let file = File::open("number.txt")?;
    let read = BufReader::new(file);
    let mut prev_digits: Vec<u32> = Vec::new();
    let mut current_product: u128 = 1;
    let mut current_max_product: u128 = 0;
    for str in read.lines() {
        let line = str;
        for char in line?.chars() {
            if let Some(digit) = char.to_digit(10) {
                if digit == 0 {
                    current_product = 1;
                    prev_digits = Vec::new();
                }
                else {
                    prev_digits.push(digit);
                    current_product = current_product * digit as u128;
                    if prev_digits.len() == 14 {
                        let last = prev_digits.remove(0);
                        current_product = current_product / last as u128;
                    }
                    if prev_digits.len() == 13 && current_max_product < current_product {
                        current_max_product = current_product;
                    }
                }
            }
        }
    }
    println!("The max product is: {}",current_max_product);
    Ok(())
}