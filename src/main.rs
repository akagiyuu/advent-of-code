#![feature(iter_array_chunks)]
mod AoC2022;
pub mod util;

const INPUT: &str = include_str!("input.txt");

fn main() {
    println!(
        "{}",
        AoC2022::day_11::calculate_monkey_bussiness_level(INPUT, 20, 3)
    );
    println!(
        "{}",
        AoC2022::day_11::calculate_monkey_bussiness_level(INPUT, 10000, 1)
    );
}
