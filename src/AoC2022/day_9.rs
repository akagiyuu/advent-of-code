use std::{collections::HashSet, str::FromStr};

use anyhow::bail;

pub type Point = (i32, i32);

fn follow_prev(current: &mut Point, prev: Point) {
    let diff_x = prev.0 - current.0;
    let diff_y = prev.1 - current.1;
    let distance_square = diff_x.pow(2) + diff_y.pow(2);

    if distance_square <= 2 {
        return;
    }
    current.0 += match diff_x {
        2 | 1 => 1,
        0 => 0,
        -2 | -1 => -1,
        _ => 0,
    };
    current.1 += match diff_y {
        2 | 1 => 1,
        0 => 0,
        -2 | -1 => -1,
        _ => 0,
    };
}

pub struct Motion {
    pub direction: u8,
    pub steps: usize,
}
impl FromStr for Motion {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(' ') {
            Some((direction, steps)) => Ok(Self {
                direction: direction.as_bytes()[0],
                steps: steps.parse()?,
            }),
            None => bail!("Invalid input"),
        }
    }
}

pub fn visited_position_count<const N: usize>(input: &str) -> usize {
    let mut visited_positions = HashSet::new();
    let mut rope = [(0, 0); N];
    visited_positions.insert(rope[N - 1]);

    for motion in input.lines().map(|line| Motion::from_str(line).unwrap()) {
        for _ in 0..motion.steps {
            match motion.direction {
                b'L' => rope[0].0 -= 1,
                b'R' => rope[0].0 += 1,
                b'U' => rope[0].1 += 1,
                b'D' => rope[0].1 -= 1,
                _ => {}
            }
            for i in 1..N {
                let prev = rope[i - 1];
                follow_prev(&mut rope[i], prev);
            }
            visited_positions.insert(rope[N - 1]);
        }
    }
    visited_positions.len()
}
