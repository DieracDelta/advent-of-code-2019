use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::str::FromStr;

fn main() {
    println!("solution day 1 part 1: {:?}", run_day_1_part_1().unwrap());
    println!("solution day 1 part 2: {:?}", run_day_1_part_2().unwrap());
}

pub fn run_day_1_part_1() -> io::Result<usize> {
    Ok(BufReader::new(File::open("./input/input-day-1.txt")?)
        .lines()
        .fold(0, |x, y| x + usize::from_str(&y.unwrap()).unwrap() / 3 - 2))
}

pub fn run_day_1_part_2() -> io::Result<usize> {
    Ok(BufReader::new(File::open("./input/input-day-1.txt")?)
        .lines()
        .fold(0, |x, y| {
            let mut s_val: isize = isize::from_str(&y.unwrap()).unwrap();
            let mut res: isize = 0;
            loop {
                s_val = (s_val / 3) - 2;
                if s_val <= 0 {
                    break;
                }
                res += s_val;
            }
            x + (res as usize)
        }))
}
