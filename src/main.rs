mod aoc;
mod utils;

use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
//    println!("{:?}", args);
    let path = args.get(1).unwrap();
//    let now = SystemTime::now();
    println!("day 1 part 1: {:?}", aoc::day1::part1(path));
    println!("day 1 part 2: {}", aoc::day1::part2(path));
    println!("day 2 part 1: {}", aoc::day2::part1(path));
  println!("day 2 part 2: {}", aoc::day2::part2(path));
}
