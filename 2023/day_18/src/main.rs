#![feature(array_windows)]
use std::str::FromStr;

use helpers::Direction;

const INPUT: &str = include_str!("input.txt");

#[derive(Clone)]
struct Instruction {
    direction: Direction,
    count: usize,
}
impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = s.as_bytes()[0].into();
        let s = &s[2..];
        let (length, _) = s.split_once(' ').unwrap();
        let length = length.parse()?;
        Ok(Instruction {
            direction,
            count: length,
        })
    }
}
impl Instruction {
    fn from_hex(hex: &str) -> anyhow::Result<Instruction> {
        let count = usize::from_str_radix(&hex[1..6], 16)?;
        let direction = hex.as_bytes()[6].into();
        Ok(Instruction { direction, count })
    }
}

fn calculate_digged_area(instructions: &[Instruction]) -> usize {
    let mut area = 0;
    let mut perimeter = 0;
    let mut coordinate = (1 << 30, 1 << 30);

    for Instruction { direction, count } in instructions {
        let new_coordinate = direction.unchecked_apply(coordinate, *count);

        area += (new_coordinate.1 * coordinate.0) as i64 - (new_coordinate.0 * coordinate.1) as i64;
        perimeter += count;

        coordinate = new_coordinate;
    }
    let area = area.unsigned_abs() as usize / 2;
    let internal_trenchs_count = area - perimeter / 2 + 1;
    perimeter + internal_trenchs_count
}

fn main() {
    let instructions = INPUT
        .trim()
        .lines()
        .map(|line| line.parse::<Instruction>().unwrap())
        .collect::<Vec<_>>();
    let area = calculate_digged_area(&instructions);
    println!("{}", area);

    let instructions = INPUT
        .trim()
        .lines()
        .map(|line| {
            let hex = line.split(' ').nth(2).unwrap();
            let hex = &hex[1..hex.len() - 1];
            Instruction::from_hex(hex).unwrap()
        })
        .collect::<Vec<_>>();
    let area = calculate_digged_area(&instructions);
    println!("{}", area);
}
