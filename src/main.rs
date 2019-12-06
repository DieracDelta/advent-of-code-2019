mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod utils;
use std::env;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = usize::from_str(&args[1]).unwrap();
    let part = usize::from_str(&args[2]).unwrap();
    let fail_str = "this part is not implemented yet".to_string();
    let result = match day {
        1 => match part {
            1 => day1::part_1().unwrap().to_string(),
            2 => day1::part_2().unwrap().to_string(),
            _ => fail_str,
        },
        2 => match part {
            1 => day2::part_1().unwrap().to_string(),
            2 => day2::part_2().unwrap().to_string(),
            _ => fail_str,
        },
        3 => match part {
            1 => day3::part_1().unwrap().to_string(),
            2 => day3::part_2().unwrap().to_string(),
            _ => fail_str,
        },
        4 => match part {
            1 => day4::part_1().unwrap().to_string(),
            2 => day4::part_2().unwrap().to_string(),
            _ => fail_str,
        },
        5 => match part {
            1 => day5::part_1().unwrap().to_string(),
            2 => day5::part_2().unwrap().to_string(),
            _ => fail_str,
        },
        6 => match part {
            1 => day6::part_1().unwrap().to_string(),
            2 => day6::part_2().unwrap().to_string(),
            _ => fail_str,
        },
        _ => fail_str,
    };
    println!(
        "for day {:?} part {:?}, the result is: {:?}",
        day, part, result
    );
}
