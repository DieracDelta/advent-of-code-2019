use crate::day2;
use crate::utils;
use std::io::{self};

pub fn part_1() -> io::Result<usize> {
    let _input_data = utils::parse(
        day2::parse_input("./input/input-day-5.txt").unwrap(),
        vec![1],
    );
    Ok(5)
}
pub fn part_2() -> io::Result<usize> {
    Ok(utils::parse(
        day2::parse_input("./input/input-day-5.txt").unwrap(),
        vec![8],
    ) as usize)
}
