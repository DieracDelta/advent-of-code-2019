use crate::day2;
use crate::utils;
use itertools::*;
use std::convert::TryFrom;
use std::fs::*;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::str::FromStr;

pub fn part_1() -> io::Result<usize> {
    let parsed = read_to_string("./input/input-day-8.txt")
        .unwrap()
        .chars()
        .take_while(|x| *x != '\n')
        .map(|x| -> u8 { char::to_digit(x, 10).unwrap() as u8 })
        .collect::<Vec<u8>>();
    Ok(parsed
        .chunks(25 * 6)
        .map(|x| -> (usize, usize) {
            let y = x.into_iter();
            let num_zero = y.clone().filter(|x| **x == 0).collect::<Vec<&u8>>().len();
            let num_one = y.clone().filter(|x| **x == 1).collect::<Vec<&u8>>().len();
            let num_two = y.filter(|x| **x == 2).collect::<Vec<&u8>>().len();
            (num_zero, (num_one as usize) * (num_two as usize))
        })
        .fold(
            (usize::max_value(), usize::max_value()),
            |acc: (usize, usize), val: (usize, usize)| -> (usize, usize) {
                if val.0 < acc.0 {
                    val
                } else {
                    acc
                }
            },
        )
        .1)
    //.collect::<Vec<(usize, usize)>>().fold;
}

pub fn part_2() -> io::Result<usize> {
    let parsed = read_to_string("./input/input-day-8.txt")
        .unwrap()
        .chars()
        .take_while(|x| *x != '\n')
        .map(|x| -> u8 { char::to_digit(x, 10).unwrap() as u8 })
        .collect::<Vec<u8>>();
    (0..25 * 6)
        .map(|index| -> u8 {
            *parsed
                .iter()
                .skip(index)
                .step_by(25 * 6)
                .find(|&&x| x == 0 || x == 1)
                .unwrap() as u8
        })
        .collect::<Vec<u8>>()
        .chunks(25)
        .map(|x| {
            println!(
                "{:?}",
                x.into_iter()
                    .map(|&y| -> String {
                        if y == 1 {
                            "X".to_owned()
                        } else {
                            " ".to_owned()
                        }
                    })
                    .collect::<String>()
            );
        })
        .last();
    Ok(0)
}
