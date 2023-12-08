use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}
impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        match value {
            b'R' => Direction::Right,
            b'L' => Direction::Left,
            _ => panic!("Invalid direction"),
        }
    }
}

fn count_steps(map: &HashMap<&[u8], [&[u8]; 2]>, directions: &[Direction]) -> usize {
    let mut current = b"AAA".as_slice();
    let mut count = 0;
    for direction in directions.iter().cycle() {
        current = match direction {
            Direction::Left => map[current][0],
            Direction::Right => map[current][1],
        };
        count += 1;
        if current == b"ZZZ" {
            break;
        }
    }
    count
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn count_steps2(map: &HashMap<&[u8], [&[u8]; 2]>, directions: &[Direction]) -> usize {
    map.iter()
        .filter(|element| element.0[2] == b'A')
        .map(|(start, _)| {
            let mut current = *start;
            let mut count = 0;
            for direction in directions.iter().cycle() {
                current = match direction {
                    Direction::Left => map[current][0],
                    Direction::Right => map[current][1],
                };
                count += 1;
                if current[2] == b'Z' {
                    break;
                }
            }
            count
        })
        .fold(1, lcm)
}

fn main() {
    let mut input = INPUT.lines();
    let directions = input
        .next()
        .unwrap()
        .bytes()
        .map(Direction::from)
        .collect::<Vec<_>>();
    input.next();
    let map = input
        .map(|line| line.as_bytes())
        .map(|line| (&line[..3], [&line[7..10], &line[12..15]]))
        .collect::<HashMap<_, _>>();
    println!("{}", count_steps(&map, &directions));
    println!("{}", count_steps2(&map, &directions));
}
