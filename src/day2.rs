use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::str::FromStr;

enum Opcode {
    Add = 1,
    Multiply = 2,
    Halt = 99,
    Unknown,
}

impl From<usize> for Opcode {
    fn from(val: usize) -> Opcode {
        match val {
            1 => Opcode::Add,
            2 => Opcode::Multiply,
            99 => Opcode::Halt,
            _ => Opcode::Unknown,
        }
    }
}

pub fn parse_input() -> io::Result<Vec<usize>> {
    Ok(BufReader::new(File::open("./input/input-day-2.txt")?)
        .lines()
        .map(|a| -> Vec<usize> {
            a.unwrap()
                .split(",")
                .map(|x| -> usize { usize::from_str(x).unwrap() })
                .collect::<Vec<usize>>()
        })
        .fold(
            vec![],
            |mut x: Vec<usize>, mut y: Vec<usize>| -> Vec<usize> {
                x.append(&mut y);
                x
            },
        ))
}

pub fn part_1() -> io::Result<usize> {
    let mut input_data = parse_input().unwrap();
    input_data[1] = 12;
    input_data[2] = 2;
    Ok(find_input(input_data))
}

pub fn part_2() -> io::Result<usize> {
    let input_data = parse_input().unwrap();
    for i in 0..100 {
        for j in 0..100 {
            let mut copy = input_data.to_vec();
            copy[1] = i;
            copy[2] = j;
            if find_input(copy) == 19690720 {
                return Ok(100 * i + j);
            }
        }
    }
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
            Opcode::Unknown => {
                println!("bad news bears");
                break;
            }
        }
    }
    input_data[0]
}
