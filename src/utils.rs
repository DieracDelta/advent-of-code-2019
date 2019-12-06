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
                Mode::Position => input[i as usize],
                _ => panic!("unknown mode"),
            }
        })
        .collect::<Vec<isize>>()
}

pub fn parse(mut input: Vec<isize>, user_input: isize) -> isize {
    let mut pc = 0;
    loop {
        let oc = Opcode::from(input[pc] as usize);
        let modes = (0..=1)
            .map(|i: u32| -> Mode {
                Mode::from(((input[pc] as usize) / (100 * ((10 as usize).pow(i))) % 10) as u8)
            })
            .collect::<Vec<Mode>>();

        //println!("\n\nboard: {:?}", input);
        match oc {
            Opcode::Add => {
                let indices = get_indices(&input, modes, input[pc + 1..=pc + 2].to_vec());
                let a = input[pc + 3] as usize;
                input[a] = indices[0] + indices[1];
                pc = pc + 4;
            }
            Opcode::Multiply => {
                let indices = get_indices(&input, modes, input[pc + 1..=pc + 2].to_vec());
                let a = input[pc + 3] as usize;
                input[a] = indices[0] * indices[1];
                pc = pc + 4;
            }
            Opcode::Halt => {
                println!("halting");
                return -1;
            }
            Opcode::Output => {
                let indices = get_indices(&input, modes, vec![input[pc + 1]])[0];
                println!("output: {:?}", indices);
                pc = pc + 2;
            }
            Opcode::Store => {
                let a = input[pc + 1] as usize;
                input[a] = user_input;
                pc = pc + 2;
            }
            Opcode::JIT => {
                let indices = get_indices(&input, modes, input[pc + 1..=pc + 2].to_vec());
                if indices[0] != 0 {
                    pc = indices[1] as usize;
                } else {
                    pc += 3
                }
            }
            Opcode::JIF => {
                let indices = get_indices(&input, modes, input[pc + 1..=pc + 2].to_vec());
                if indices[0] == 0 {
                    pc = indices[1] as usize;
                } else {
                    pc += 3
                }
            }
            Opcode::LT => {
                let indices = get_indices(&input, modes, input[pc + 1..=pc + 2].to_vec());
                let a = input[pc + 3] as usize;
                input[a] = (indices[0] < indices[1]) as isize;
                pc += 4;
            }
            Opcode::EQ => {
                let indices = get_indices(&input, modes, input[pc + 1..=pc + 2].to_vec());
                let a = input[pc + 3] as usize;
                input[a] = (indices[0] == indices[1]) as isize;
                pc += 4;
            }
            _ => {
                panic!("Opcode: {:?}, {:?}", input[pc] % 100, oc);
            }
        }
    }
    panic!("bad news");
}
