#![feature(iter_array_chunks)]
mod AoC2022;

const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("{}", AoC2022::day_10::calculate_signal_stregth(INPUT));
    println!("{}", AoC2022::day_10::render(INPUT));
}
