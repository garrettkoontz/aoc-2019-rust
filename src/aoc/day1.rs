use crate::utils;

const FILE_NAME: &str = "day1.txt";

pub fn part1(path: &str) -> i32 {
    let file_path = &format!("./{}/{}", path, FILE_NAME);
    let inputs = utils::read_file(file_path);
    inputs
        .into_iter()
        .map(|x| fuel_required(x.parse::<i32>().unwrap()))
        .fold(0, |acc, x| acc + x)
}

pub fn part2(path: &str) -> i32 {
    let file_path = &format!("{}/{}", path, FILE_NAME);
    let inputs = utils::read_file(file_path);
    inputs
        .into_iter()
        .map(|x| fuel_required_with_fuel(x.parse::<i32>().unwrap()))
        .fold(0, |acc, x| acc + x)
}

fn fuel_required(i: i32) -> i32 {
    (i / 3) - 2
}

fn fuel_required_with_fuel(i: i32) -> i32 {
    let mut sum = 0;
    let mut mass = i;
    while mass > 0 {
        let new_m = fuel_required(mass);
        sum += if new_m > 0 { new_m } else { 0 };
        mass = new_m;
    }
    sum
}
