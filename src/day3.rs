use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::iter::FromIterator;
use std::str::FromStr;

#[derive(Clone, Debug, Copy)]
pub struct Step {
    direction: Direction,
    magnitude: isize,
}

#[derive(Clone, Copy, Debug)]
pub struct Line {
    x1: isize,
    y1: isize,
    x2: isize,
    y2: isize,
}

#[derive(Clone, Copy, Debug)]
pub struct CurState {
    x: isize,
    y: isize,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
    Unknown,
}

impl From<u8> for Direction {
    fn from(val: u8) -> Direction {
        match val {
            b'R' => Direction::Right,
            b'L' => Direction::Left,
            b'U' => Direction::Up,
            b'D' => Direction::Down,
            _ => Direction::Unknown,
        }
    }
}

pub fn parse_input() -> io::Result<Vec<Vec<Step>>> {
    Ok(BufReader::new(File::open("./input/input-day-3.txt")?)
        .lines()
        .map(|a| -> Vec<Step> {
            a.unwrap()
                .split(",")
                .map(|x: &str| -> Step {
                    Step {
                        direction: Direction::from(x.as_bytes()[0]),
                        magnitude: isize::from_str(
                            std::str::from_utf8(&x.as_bytes()[1..]).unwrap(),
                        )
                        .unwrap(),
                    }
                })
                .collect::<Vec<Step>>()
        })
        .collect())
}

pub fn increment_current_state(cur: &mut CurState, step: &Step) {
    match step.direction {
        Direction::Right => cur.x += step.magnitude,
        Direction::Left => {
            cur.x -= step.magnitude;
        }
        Direction::Up => cur.y += step.magnitude,
        Direction::Down => {
            cur.y -= step.magnitude;
        }
        Direction::Unknown => panic!("unknown direction"),
    }
}

pub fn get_lines<'a>(input: Vec<Step>) -> Vec<Line> {
    let start_state = &mut CurState { x: 0, y: 0 };
    input
        .iter()
        .map(|x: &Step| -> Line {
            let temp_state = start_state.clone();
            increment_current_state(start_state, x);
            Line {
                x1: temp_state.x,
                y1: temp_state.y,
                x2: start_state.x,
                y2: start_state.y,
            }
        })
        .collect::<Vec<Line>>()
}

pub fn part_1() -> io::Result<usize> {
    let input_data = parse_input().unwrap();
    let player_1 = get_lines(input_data[0].clone());
    let player_2 = get_lines(input_data[1].clone());
    let p1_hm = &mut std::collections::HashMap::<(isize, isize), usize>::new();
    let p2_hm = &mut std::collections::HashMap::<(isize, isize), usize>::new();
    construct_hashmap(&player_1, p1_hm);
    construct_hashmap(&player_2, p2_hm);
    p1_hm.remove(&(0, 0));
    p2_hm.remove(&(0, 0));
    let set_1 = HashSet::<(isize, isize)>::from_iter(p1_hm.keys().cloned());
    let set_2 = HashSet::<(isize, isize)>::from_iter(p2_hm.keys().cloned());
    Ok(set_1
        .intersection(&set_2)
        .map(|a| -> usize { a.0.abs() as usize + a.1.abs() as usize })
        .min()
        .unwrap())
}

pub fn construct_hashmap(
    lines: &Vec<Line>,
    hm: &mut std::collections::HashMap<(isize, isize), usize>,
) {
    let mut counter: usize = 0;
    for line in lines {
        if line.x1 == line.x2 {
            if line.y1 < line.y2 {
                for y in line.y1..(line.y2) {
                    if !hm.contains_key(&(line.x1, y)) {
                        hm.insert((line.x1, y), counter);
                    }
                    counter += 1;
                }
            } else {
                for y in (line.y2..line.y1).rev() {
                    if !hm.contains_key(&(line.x1, y)) {
                        hm.insert((line.x1, y), counter);
                    }
                    counter += 1;
                }
            }
        } else if line.y1 == line.y2 {
            if line.x1 < line.x2 {
                for x in line.x1..line.x2 {
                    if !hm.contains_key(&(x, line.y1)) {
                        hm.insert((x, line.y1), counter);
                    }
                    counter += 1;
                }
            } else {
                for x in (line.x2..line.x1).rev() {
                    if !hm.contains_key(&(x, line.y1)) {
                        hm.insert((x, line.y1), counter);
                    }
                    counter += 1;
                }
            }
        } else {
            panic!("SHIT");
        }
    }
}

pub fn part_2() -> io::Result<usize> {
    let input_data = parse_input().unwrap();
    let player_1 = get_lines(input_data[0].clone());
    let player_2 = get_lines(input_data[1].clone());
    let p1_hm = &mut std::collections::HashMap::<(isize, isize), usize>::new();
    let p2_hm = &mut std::collections::HashMap::<(isize, isize), usize>::new();
    construct_hashmap(&player_1, p1_hm);
    construct_hashmap(&player_2, p2_hm);
    p1_hm.remove(&(0, 0));
    p2_hm.remove(&(0, 0));
    let set_1 = HashSet::<(isize, isize)>::from_iter(p1_hm.keys().cloned());
    let set_2 = HashSet::<(isize, isize)>::from_iter(p2_hm.keys().cloned());
    Ok(set_1
        .intersection(&set_2)
        .map(|a| -> usize { p1_hm[a] + p2_hm[a] })
        .min()
        .unwrap())
}
