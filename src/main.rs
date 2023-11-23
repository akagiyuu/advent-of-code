#![feature(iter_array_chunks)]
#![allow(dead_code)]
#![allow(non_snake_case)]
mod AoC2022;
pub mod util;

const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("{}", AoC2022::day_12::find_min_step_1(INPUT));
    println!("{}", AoC2022::day_12::find_min_step_2(INPUT));
}
