use crate::day2;
use crate::utils;
use std::convert::TryFrom;
use std::io::{self};

pub fn part_1() -> io::Result<usize> {
    let program = day2::parse_input("./input/input-day-7.txt").unwrap();
    let mut rval = 0;
    for i in 0..=4 {
        for j in 0..=4 {
            for k in 0..=4 {
                for m in 0..=4 {
                    for n in 0..=4 {
                        if i == j
                            || i == k
                            || i == m
                            || i == n
                            || j == k
                            || j == m
                            || j == n
                            || k == m
                            || k == n
                            || m == n
                        {
                            continue;
                        }
                        let new_val =
                            usize::try_from(run_thruster(vec![i, j, k, m, n], program.clone()))
                                .unwrap();
                        println!(
                            "combo: {:?}{:?}{:?}{:?}{:?} val: {:?}",
                            i, j, k, m, n, new_val
                        );
                        if new_val > rval {
                            rval = new_val;
                        }
                    }
                }
            }
        }
    }
    Ok(rval)
}
pub fn part_2() -> io::Result<usize> {
    let program = day2::parse_input("./input/input-day-7.txt").unwrap();
    let mut rval = 0;
    for i in 5..=9 {
        for j in 5..=9 {
            for k in 5..=9 {
                for m in 5..=9 {
                    for n in 5..=9 {
                        if i == j
                            || i == k
                            || i == m
                            || i == n
                            || j == k
                            || j == m
                            || j == n
                            || k == m
                            || k == n
                            || m == n
                        {
                            continue;
                        }
                        let new_val = usize::try_from(run_thruster_loop(
                            vec![i, j, k, m, n],
                            program.clone(),
                        ))
                        .unwrap();
                        println!(
                            "combo: {:?}{:?}{:?}{:?}{:?} val: {:?}",
                            i, j, k, m, n, new_val
                        );
                        if new_val > rval {
                            rval = new_val;
                        }
                    }
                }
            }
        }
    }
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
                            println!("output: {:?}", output);
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
