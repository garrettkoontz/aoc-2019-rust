pub struct IntCodeComputer {
    input_arr: Vec<i32>,
}

fn execute_program(int_code_computer: IntCodeComputer, noun: i32, verb: i32) -> Vec<i32> {
    let mut memory: Vec<i32> = Vec::new();
    memory[..0].clone_from_slice(&int_code_computer.input_arr);
    if noun >= 0 {
        memory[1] = noun;
    }
    if verb >= 0 {
        memory[2] = verb;
    }
    let mut ip = 0;
    let mut ins = parse_instruction(int_code_computer.input_arr, ip);
    while !ip.get_halt() {
        ip = ins.execute_instruction(memory, ip);
        ins = parse_instruction(memory, ip);
    }
    memory
}

fn execute_instruction(arr: &mut Vec<i32>, ins: impl Instruction, ip: usize) -> usize {
    ins.execute_instruction(arr, ip)
}

fn parse_instruction(arr: &Vec<i32>, i: usize) -> impl Instruction {
    match arr[i] {
        1 => Add {
            p1: arr[i + 1],
            p2: arr[i + 2],
            p_out: arr[i + 3],
        },
        2 => Mul {
            p1: arr[i + 1],
            p2: arr[i + 2],
            p_out: arr[i + 3],
        },
        99 => Hlt {},
    }
}

pub trait Instruction {
    fn get_halt(&self) -> boolean {
        false
    }
    fn execute_instruction(&self, memory: &mut Vec<i32>, ip: usize) -> usize;
}

pub struct Add {
    p1: i32,
    p2: i32,
    p_out: i32,
}

pub struct Mul {
    p1: i32,
    p2: i32,
    p_out: i32,
}

pub struct Hlt {}

impl Instruction for Add {
    fn execute_instruction(&self, memory: &mut Vec<i32>, ip: usize) -> usize {
        memory[self.p_out] = memory[self.p1] + memory[self.p2];
        ip + 4
    }
}

impl Instruction for Mul {
    fn execute_instruction(&self, memory: &mut Vec<i32>, ip: usize) -> usize {
        memory[self.p_out] = memory[self.p1] * memory[self.p2];
        ip + 4
    }
}

impl Instruction for Hlt {
    fn execute_instruction(&self, memory: &mut Vec<i32>, ip: usize) -> usize {
        ip + 1
    }
    fn get_halt(&self) -> boolean {
        true
    }
}
