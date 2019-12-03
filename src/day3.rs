use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::str::FromStr;

#[derive(Clone)]
pub struct Step {
    direction: Direction,
    magnitude: usize,
}

#[derive(Clone, Debug)]
pub struct CurState {
    x: usize,
    y: usize,
}

#[derive(Clone)]
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
                        magnitude: usize::from_str(
                            std::str::from_utf8(&x.as_bytes()[1..]).unwrap(),
                        )
                        .unwrap(),
                    }
                })
                .collect::<Vec<Step>>()
        })
        .collect())
}

pub fn increment_current_state(cur: &mut CurState, step: Step, start_state: &mut CurState) {
    match step.direction {
        Direction::Right => cur.x += step.magnitude,
        Direction::Left => {
            if cur.x < step.magnitude {
                start_state.x += step.magnitude - cur.x;
                cur.x = 0;
            } else {
                cur.x -= step.magnitude;
            }
        }
        Direction::Up => cur.y += step.magnitude,
        Direction::Down => {
            if cur.y < step.magnitude {
                start_state.y += step.magnitude - cur.y;
                cur.y = 0;
            } else {
                cur.y -= step.magnitude;
            }
        }
        Direction::Unknown => panic!("unknown direction"),
    }
}

pub fn inc_and_mark(cur: &mut CurState, step: Step, state: &mut Vec<Vec<u8>>) {
    match step.direction {
        Direction::Right => {
            for i in 1..step.magnitude + 1 {
                state[cur.x + i][cur.y] += 1;
            }
            cur.x += step.magnitude;
        }
        Direction::Left => {
            for i in 1..step.magnitude + 1 {
                state[cur.x - i][cur.y] += 1;
            }
            cur.x -= step.magnitude;
        }
        Direction::Up => {
            for i in 1..step.magnitude + 1 {
                state[cur.x][cur.y + i] += 1;
            }
            cur.y += step.magnitude;
        }
        Direction::Down => {
            for i in 1..step.magnitude + 1 {
                state[cur.x][cur.y - i] += 1;
            }
            cur.y -= step.magnitude;
        }
        Direction::Unknown => panic!("unknown direction"),
    }
}

pub fn find_dims(input_data: &Vec<Vec<Step>>, mut start_state: &mut CurState) -> (usize, usize) {
    let mut max_x = 0;
    let mut max_y = 0;
    for vec in input_data {
        let mut state = CurState { x: 0, y: 0 };
        for step in vec {
            increment_current_state(&mut state, step.clone(), &mut start_state);
            if state.x > max_x {
                max_x = state.x
            }
            if state.y > max_y {
                max_y = state.y
            }
        }
    }
    (max_x, max_y)
}

pub fn mark(input_data: &Vec<Step>, state: &mut Vec<Vec<u8>>, mut start_state: CurState) {
    for entry in input_data {
        inc_and_mark(&mut start_state, entry.clone(), state);
    }
}

pub fn part_1() -> io::Result<usize> {
    let input_data = parse_input().unwrap();
    let mut start_state = CurState { x: 0, y: 0 };
    let dims = find_dims(&input_data, &mut start_state);
    let mut state = vec![vec![0u8; dims.1]; dims.0];
    &input_data
        .into_iter()
        .map(|x| mark(&x, &mut state, start_state.clone()));

    let mut man_dist = usize::max_value();
    for x in 0..dims.0 {
        for y in 0..dims.1 {
            if state[x][y] == 2 && x + y < man_dist {
                println!("hi");
                man_dist = x + y;
            }
        }
    }
    Ok(man_dist)
}

pub fn part_2() -> io::Result<usize> {
    Ok(5)
}
