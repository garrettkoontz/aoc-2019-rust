use crate::aoc::int_code_computer::IntCodeComputer;
use crate::utils;

const FILE_NAME: &str = "day2.txt";

pub fn part1(path: &str) -> usize {
    let file_path = &format!("./{}/{}", path, FILE_NAME);
    let inputs = utils::read_file(file_path);
    let input_memory = inputs[0]
        .split(",")
        .into_iter()
        .map(|x| x.trim().parse::<usize>().unwrap())
        .collect();
    let icc = IntCodeComputer {
        input_arr: input_memory,
    };
    icc.execute_program(12, 2)[0]
}

pub fn part2(path: &str) -> i32 {
    let file_path = &format!("./{}/{}", path, FILE_NAME);
    let inputs = utils::read_file(file_path);
    let input_memory = inputs[0]
        .split(",")
        .into_iter()
        .map(|x| x.trim().parse::<usize>().unwrap())
        .collect();
    let icc = IntCodeComputer {
        input_arr: input_memory,
    };
    for noun in 0..99 {
        for verb in 0..99 {
            if icc.execute_program(noun, verb)[0] == 19690720 {
                return (noun * 100 + verb) as i32;
            }
        }
    }
    return -1;
}
