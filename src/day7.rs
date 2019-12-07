use crate::day2;
use crate::utils;
use itertools::*;
use std::convert::TryFrom;
use std::io::{self};

pub fn part_1() -> io::Result<usize> {
    let program = day2::parse_input("./input/input-day-7.txt").unwrap();
    let mut rval = 0;
    iproduct!(5..=9, 5..=9, 5..=9, 5..=9, 5..=9)
        .filter(|x| {
            vec![x.0, x.1, x.2, x.3, x.4]
                .into_iter()
                .unique()
                .collect::<Vec<_>>()
                .len()
                == 5
        })
        .for_each(|x| {
            let new_val =
                usize::try_from(run_thruster(vec![x.0, x.1, x.2, x.3, x.4], program.clone()))
                    .unwrap();
            if new_val > rval {
                rval = new_val;
            }
        });
    Ok(rval)
}

pub fn part_2() -> io::Result<usize> {
    let program = day2::parse_input("./input/input-day-7.txt").unwrap();
    let mut rval = 0;
    iproduct!(5..=9, 5..=9, 5..=9, 5..=9, 5..=9)
        .filter(|x| {
            vec![x.0, x.1, x.2, x.3, x.4]
                .into_iter()
                .unique()
                .collect::<Vec<_>>()
                .len()
                == 5
        })
        .for_each(|x| {
            let new_val = usize::try_from(run_thruster_loop(
                vec![x.0, x.1, x.2, x.3, x.4],
                program.clone(),
            ))
            .unwrap();
            if new_val > rval {
                rval = new_val;
            }
        });
    Ok(rval)
}

pub fn run_thruster_loop(thruster_ids: Vec<isize>, program: Vec<isize>) -> isize {
    let mut pc_list: Vec<usize> = (0..=4)
        .into_iter()
        .map(|_| -> usize { 0 })
        .collect::<Vec<usize>>();
    let mut programs = (0..=4)
        .into_iter()
        .map(|_| -> Vec<isize> { program.clone() })
        .collect::<Vec<Vec<isize>>>();
    let mut most_recent_output = 0;
    let mut first_time = true;
    let mut is_done = false;
    loop {
        most_recent_output =
            (0..=4)
                .into_iter()
                .fold(most_recent_output, |acc: isize, index: usize| -> isize {
                    match utils::run_to_output_or_halt(
                        &mut programs[index],
                        pc_list[index],
                        if first_time {
                            vec![acc, thruster_ids[index].clone()]
                        } else {
                            vec![acc]
                        },
                    ) {
                        Some((output, pc)) => {
                            pc_list[index] = pc;
                            output
                        }
                        None => {
                            is_done = true;
                            acc
                        }
                    }
                });
        first_time = false;
        if is_done {
            return most_recent_output;
        }
    }
}

pub fn run_thruster(thruster_ids: Vec<isize>, program: Vec<isize>) -> isize {
    (0..=4)
        .into_iter()
        .fold(0, |acc: isize, index: isize| -> isize {
            utils::parse(
                program.clone(),
                vec![acc, thruster_ids[index as usize].clone()],
            )
        })
}
