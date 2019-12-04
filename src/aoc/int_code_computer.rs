pub struct IntCodeComputer {
    pub input_arr: Vec<usize>,
}

impl IntCodeComputer {
    pub fn execute_program(&self, noun: usize, verb: usize) -> Vec<usize> {
        let mut memory: Vec<usize> = self.input_arr.to_vec();
        memory[1] = noun;
        memory[2] = verb;
        let mut ip = 0;
        let mut ins = parse_instruction(&self.input_arr, ip);
        while !ins.get_halt() {
            ip = ins.execute_instruction(&mut memory, ip);
            ins = parse_instruction(&memory, ip);
        }
        memory
    }
}

fn parse_instruction(arr: &Vec<usize>, i: usize) -> Ins {
    match arr[i] {
        1 => Ins::ADD(Add {
            p1: arr[i + 1 as usize],
            p2: arr[i + 2 as usize],
            p_out: arr[i + 3 as usize],
        }),
        2 => Ins::MUL(Mul {
            p1: arr[i + 1 as usize],
            p2: arr[i + 2 as usize],
            p_out: arr[i + 3 as usize],
        }),
        99 => Ins::HLT(Hlt {}),
        _ => panic!("Unable to determine what to do!"),
    }
}

pub enum Ins {
    ADD(Add),
    MUL(Mul),
    HLT(Hlt),
}

pub trait Instruction {
    fn get_halt(&self) -> bool {
        false
    }
    fn execute_instruction(&self, memory: &mut Vec<usize>, ip: usize) -> usize;
}

pub struct Add {
    p1: usize,
    p2: usize,
    p_out: usize,
}

pub struct Mul {
    p1: usize,
    p2: usize,
    p_out: usize,
}

pub struct Hlt {}

impl Instruction for Ins {
    fn get_halt(&self) -> bool {
        match &self {
            Ins::HLT(_) => true,
            _ => false,
        }
    }

    fn execute_instruction(&self, memory: &mut Vec<usize>, ip: usize) -> usize {
        match *self {
            Ins::ADD(ref data) => data.execute_instruction(memory, ip),
            Ins::MUL(ref data) => data.execute_instruction(memory, ip),
            Ins::HLT(ref data) => data.execute_instruction(memory, ip),
        }
    }
}

impl Instruction for Add {
    fn execute_instruction(&self, memory: &mut Vec<usize>, ip: usize) -> usize {
        memory[self.p_out] = memory[self.p1] + memory[self.p2];
        ip + 4
    }
}

impl Instruction for Mul {
    fn execute_instruction(&self, memory: &mut Vec<usize>, ip: usize) -> usize {
        memory[self.p_out] = memory[self.p1] * memory[self.p2];
        ip + 4
    }
}

impl Instruction for Hlt {
    fn execute_instruction(&self, _memory: &mut Vec<usize>, ip: usize) -> usize {
        ip + 1
    }
    fn get_halt(&self) -> bool {
        true
    }
}
