use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
//use std::str::FromStr;

pub fn part_1() -> io::Result<isize> {
    let mut input = BufReader::new(File::open("./input/input-day-16.txt")?)
        .lines()
        .next()
        .unwrap()?
        .chars()
        .map(|x| -> isize { x.to_digit(10).unwrap() as isize })
        .collect::<Vec<isize>>();
    let base_pattern = vec![0, 1, 0, -1];
    let patterns = gen_patterns(input.len(), &base_pattern);
    let mut next_input: Vec<isize>;
    for _ in 0..100 {
        next_input = Vec::new();
        input
            .iter()
            .enumerate()
            .map(|(index, _)| {
                next_input.push(get_ele(&input, &patterns[index], index));
            })
            .last();
        input = next_input.clone();
    }
    Ok(input[0..8]
        .iter()
        .fold(0, |acc, ele| -> isize { (acc * 10) + ele }))
}

pub fn part_2() -> io::Result<isize> {
    let mut input = BufReader::new(File::open("./input/input-day-16.txt")?)
        .lines()
        .next()
        .unwrap()?
        .chars()
        .map(|x| -> isize { x.to_digit(10).unwrap() as isize })
        .collect::<Vec<isize>>();
    input = input.repeat(10000);
    //let len = input.len();
    //input = input.iter().cycle().take(1000 * len).collect();
    let base_pattern = vec![0, 1, 0, -1];
    println!("generated patterns start");
    let patterns = gen_patterns(input.len(), &base_pattern);
    println!("generated patterns end");
    let mut next_input: Vec<isize>;
    for _i in 0..100 {
        println!("iteration: {:?}", _i);
        next_input = Vec::new();
        input
            .iter()
            .enumerate()
            .map(|(index, _)| {
                println!("\tele: {:?}", index);
                next_input.push(get_ele(&input, &patterns[index], index));
            })
            .last();
        input = next_input.clone();
    }
    let offset = input[0..8]
        .iter()
        .fold(0, |acc, &ele| -> usize { (acc * 10) + (ele as usize) });

    Ok(input[offset..offset + 8]
        .iter()
        .fold(0, |acc, ele| -> isize { (acc * 10) + ele }))
}

pub fn gen_patterns(lim: usize, base_pattern: &Vec<isize>) -> Vec<Vec<isize>> {
    let mut r_val = Vec::new();
    for index in 0..lim {
        println!("{:?}", index);
        r_val.push(
            base_pattern
                .iter()
                .map(|x: &isize| -> Vec<isize> { vec![*x; index + 1] })
                .flatten().collect()
                //.cycle()
                //.skip(1),
        )
    }
    r_val
}

pub fn get_ele(input: &Vec<isize>, pattern: &Vec<isize>, _index: usize) -> isize {
    // TODO precompute
    let mut r_val = input
        .iter()
        .zip(pattern.iter().cycle().skip(1))
        .fold(0, |acc, (a, b)| acc + a * b);
    if r_val < 0 {
        r_val *= -1;
    }
    r_val % 10
}
