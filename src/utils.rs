use std::convert::TryFrom;
#[derive(Clone, Debug)]
pub enum Opcode {
    Add = 1,
    Multiply = 2,
    Store = 3,
    Output = 4,
    JIT = 5,
    JIF = 6,
    LT = 7,
    EQ = 8,
    Halt = 99,
    Unknown,
}

#[derive(Clone, Debug)]
pub enum Mode {
    Position = 0,
    Immediate = 1,
    Unknown,
}

impl From<u8> for Mode {
    fn from(val: u8) -> Mode {
        match val % 100 {
            0 => Mode::Position,
            1 => Mode::Immediate,
            _ => Mode::Unknown,
        }
    }
}

impl From<usize> for Opcode {
    fn from(val: usize) -> Opcode {
        match val % 100 {
            1 => Opcode::Add,
            2 => Opcode::Multiply,
            3 => Opcode::Store,
            4 => Opcode::Output,
            5 => Opcode::JIT,
            6 => Opcode::JIF,
            7 => Opcode::LT,
            8 => Opcode::EQ,
            99 => Opcode::Halt,
            _ => Opcode::Unknown,
        }
    }
}

pub fn get_indices(input: &Vec<isize>, modes: Vec<Mode>, params: Vec<isize>) -> Vec<isize> {
    (0..params.len())
        .map(|x: usize| -> isize {
            let i: isize = params[x];
            match modes[x] {
                Mode::Immediate => i,
                Mode::Position => input[usize::try_from(i).unwrap()],
                _ => panic!("unknown mode"),
            }
        })
        .collect::<Vec<isize>>()
}

pub fn parse(mut input: Vec<isize>, mut user_input: Vec<isize>) -> isize {
    let mut pc = 0;
    let mut rval = 0;
    loop {
        let oc = Opcode::from(usize::try_from(input[pc]).unwrap());
        //print!("state: {:?}, opcode: {:?}, pc: {:?}", input, oc, pc);
        let modes = (0..=1)
            .map(|i: u32| -> Mode {
                Mode::from(
                    ((usize::try_from(input[pc]).unwrap()) / (100 * ((10 as usize).pow(i))) % 10)
                        as u8,
                )
            })
            .collect::<Vec<Mode>>();

        //println!("\n\nboard: {:?}", input);
        match oc {
            Opcode::Add => {
                let indices = get_indices(&input, modes, input[pc + 1..=pc + 2].to_vec());
                let a = usize::try_from(input[pc + 3]).unwrap();
                //println!(" arr[{:?}] = {:?} + {:?}", a, indices[0], indices[1]);
                input[a] = indices[0] + indices[1];
                pc = pc + 4;
            }
            Opcode::Multiply => {
                let indices = get_indices(&input, modes, input[pc + 1..=pc + 2].to_vec());
                let a: usize = usize::try_from(input[pc + 3]).unwrap();
                //println!(" arr[{:?}] = {:?} * {:?}", a, indices[0], indices[1]);
                input[a] = indices[0] * indices[1];
                pc = pc + 4;
            }
            Opcode::Halt => {
                println!("halt");
                //println!("halting");
                return rval;
            }
            Opcode::Output => {
                let indices = get_indices(&input, modes, vec![input[pc + 1]])[0];
                //println!(" return {:?}", indices);
                rval = indices;
                pc = pc + 2;
            }
            Opcode::Store => {
                let a: usize = usize::try_from(input[pc + 1]).unwrap();
                //println!(
                //" store {:?} at arr[{:?}]",
                //user_input[user_input.len() - 1],
                //a
                //);
                input[a] = user_input.pop().unwrap();
                pc = pc + 2;
            }
            Opcode::JIT => {
                let indices = get_indices(&input, modes, input[pc + 1..=pc + 2].to_vec());
                if indices[0] != 0 {
                    pc = usize::try_from(indices[1]).unwrap();
                } else {
                    pc += 3
                }
            }
            Opcode::JIF => {
                let indices = get_indices(&input, modes, input[pc + 1..=pc + 2].to_vec());
                if indices[0] == 0 {
                    pc = usize::try_from(indices[1]).unwrap();
                } else {
                    pc += 3
                }
            }
            Opcode::LT => {
                let indices = get_indices(&input, modes, input[pc + 1..=pc + 2].to_vec());
                let a = usize::try_from(input[pc + 3]).unwrap();
                input[a] = isize::try_from(indices[0] < indices[1]).unwrap();
                pc += 4;
            }
            Opcode::EQ => {
                let indices = get_indices(&input, modes, input[pc + 1..=pc + 2].to_vec());
                let a = usize::try_from(input[pc + 3]).unwrap();
                input[a] =
                    isize::try_from(indices[0] == isize::try_from(indices[1]).unwrap()).unwrap();
                pc += 4;
            }
            _ => {
                panic!("Opcode: {:?}, {:?}", input[pc] % 100, oc);
            }
        }
    }
}

// returns output and pc
pub fn run_to_output_or_halt(
    input: &mut Vec<isize>,
    mut pc: usize,
    mut user_input: Vec<isize>,
) -> Option<(isize, usize)> {
    loop {
        let oc = Opcode::from(usize::try_from(input[pc]).unwrap());
        let modes = (0..=1)
            .map(|i: u32| -> Mode {
                Mode::from(
                    ((usize::try_from(input[pc]).unwrap()) / (100 * ((10 as usize).pow(i))) % 10)
                        as u8,
                )
            })
            .collect::<Vec<Mode>>();

        //println!("\n\nboard: {:?}", input);
        match oc {
            Opcode::Add => {
                let indices = get_indices(&input, modes, input[pc + 1..=pc + 2].to_vec());
                let a = usize::try_from(input[pc + 3]).unwrap();
                //println!(" arr[{:?}] = {:?} + {:?}", a, indices[0], indices[1]);
                input[a] = indices[0] + indices[1];
                pc = pc + 4;
            }
            Opcode::Multiply => {
                let indices = get_indices(&input, modes, input[pc + 1..=pc + 2].to_vec());
                let a: usize = usize::try_from(input[pc + 3]).unwrap();
                //println!(" arr[{:?}] = {:?} * {:?}", a, indices[0], indices[1]);
                input[a] = indices[0] * indices[1];
                pc = pc + 4;
            }
            Opcode::Halt => return None,
            Opcode::Output => {
                let indices = get_indices(&input, modes, vec![input[pc + 1]])[0];
                //println!(" return {:?}", indices);
                pc = pc + 2;
                return Some((indices, pc));
            }
            Opcode::Store => {
                let a: usize = usize::try_from(input[pc + 1]).unwrap();
                //println!(
                //" store {:?} at arr[{:?}]",
                //user_input[user_input.len() - 1],
                //a
                //);
                input[a] = user_input.pop().unwrap();
                pc = pc + 2;
            }
            Opcode::JIT => {
                let indices = get_indices(&input, modes, input[pc + 1..=pc + 2].to_vec());
                if indices[0] != 0 {
                    pc = usize::try_from(indices[1]).unwrap();
                } else {
                    pc += 3
                }
            }
            Opcode::JIF => {
                let indices = get_indices(&input, modes, input[pc + 1..=pc + 2].to_vec());
                if indices[0] == 0 {
                    pc = usize::try_from(indices[1]).unwrap();
                } else {
                    pc += 3
                }
            }
            Opcode::LT => {
                let indices = get_indices(&input, modes, input[pc + 1..=pc + 2].to_vec());
                let a = usize::try_from(input[pc + 3]).unwrap();
                input[a] = isize::try_from(indices[0] < indices[1]).unwrap();
                pc += 4;
            }
            Opcode::EQ => {
                let indices = get_indices(&input, modes, input[pc + 1..=pc + 2].to_vec());
                let a = usize::try_from(input[pc + 3]).unwrap();
                input[a] =
                    isize::try_from(indices[0] == isize::try_from(indices[1]).unwrap()).unwrap();
                pc += 4;
            }
            _ => {
                panic!("Opcode: {:?}, {:?}", input[pc] % 100, oc);
            }
        }
    }
    None
}
