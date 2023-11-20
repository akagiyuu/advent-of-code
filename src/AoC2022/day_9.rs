use std::{collections::HashSet, str::FromStr};

use anyhow::bail;

pub type Point = (i32, i32);
fn distance_square(a: &Point, b: &Point) -> i32 {
    (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2)
}

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}
impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            _ => bail!("Invalid direction format"),
        }
    }
}
pub struct Motion {
    pub direction: Direction,
    pub steps: usize,
}
impl FromStr for Motion {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(' ') {
            Some((direction, steps)) => Ok(Self {
                direction: Direction::from_str(direction)?,
                steps: steps.parse()?,
            }),
            None => bail!("Invalid input"),
        }
    }
}

pub fn visited_position_count(motions: Vec<Motion>) -> usize {
    let mut visited_positions = HashSet::new();
    let mut head = (0, 0);
    let mut tail = (0, 0);
    visited_positions.insert(tail);
    for motion in motions {
        for _ in 0..motion.steps {
            match motion.direction {
                Direction::Left => {
                    head.0 -= 1;
                    if distance_square(&head, &tail) > 2 {
                        tail = (head.0 + 1, head.1);
                        visited_positions.insert(tail);
                    }
                }
                Direction::Right => {
                    head.0 += 1;
                    if distance_square(&head, &tail) > 2 {
                        tail = (head.0 - 1, head.1);
                        visited_positions.insert(tail);
                    }
                }
                Direction::Up => {
                    head.1 += 1;
                    if distance_square(&head, &tail) > 2 {
                        tail = (head.0, head.1 - 1);
                        visited_positions.insert(tail);
                    }
                }
                Direction::Down => {
                    head.1 -= 1;
                    if distance_square(&head, &tail) > 2 {
                        tail = (head.0, head.1 + 1);
                        visited_positions.insert(tail);
                    }
                }
            }
        }
    }
    println!("{:?}", visited_positions);

    visited_positions.len()
}
