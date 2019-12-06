use crate::utils::Opcode;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::str::FromStr;

pub fn parse_input(input_str: &str) -> io::Result<Vec<isize>> {
    Ok(BufReader::new(File::open(input_str)?)
        .lines()
        .map(|a| -> Vec<isize> {
            a.unwrap()
                .split(",")
                .map(|x| -> isize { isize::from_str(x).unwrap() })
                .collect::<Vec<isize>>()
        })
        .fold(
            vec![],
            |mut x: Vec<isize>, mut y: Vec<isize>| -> Vec<isize> {
                x.append(&mut y);
                x
            },
        ))
}

pub fn part_1() -> io::Result<usize> {
    let mut input_data = parse_input("./input/input-day-2.txt")
        .unwrap()
        .iter()
        .map(|x| -> usize { *x as usize })
        .collect::<Vec<usize>>();
    input_data[1] = 12;
    input_data[2] = 2;
    Ok(find_input(input_data) as usize)
}

pub fn part_2() -> io::Result<usize> {
    //let input_data = parse_input("./input/input-day-2.txt").unwrap();
    //for i in 0..100 {
    //for j in 0..100 {
    //let mut copy = input_data.to_vec();
    //copy[1] = i;
    //copy[2] = j;
    //if find_input(copy) == 19690720 {
    //return Ok((100 * i + j) as usize);
    //}
    //}
    //}
    Ok(0)
}

pub fn find_input(mut input_data: Vec<usize>) -> usize {
    for i in 0..input_data.len() / 4 {
        let index_0 = input_data[4 * i + 1];
        let index_1 = input_data[4 * i + 2];
        let index_2 = input_data[4 * i + 3];
        match Opcode::from(input_data[4 * i]) {
            Opcode::Add => {
                //if index_0 >= input_data.len()
                //|| index_1 >= input_data.len()
                //|| index_2 >= input_data.len()
                //{
                //return 0;
                //}
                input_data[index_2] = input_data[index_0] + input_data[index_1];
            }
            Opcode::Multiply => {
                //if index_0 >= input_data.len()
                //|| index_1 >= input_data.len()
                //|| index_2 >= input_data.len()
                //{
                //return 0;
                //}
                input_data[index_2] = input_data[index_0] * input_data[index_1];
            }
            Opcode::Halt => break,
            _ => {
                println!("bad news bears");
                break;
            }
        }
    }
    input_data[0]
}
