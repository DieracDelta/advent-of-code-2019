mod day1;
mod day2;
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
        _ => fail_str,
    };
    println!(
        "for day {:?} part {:?}, the result is: {:?}",
        day, part, result
    );
}
