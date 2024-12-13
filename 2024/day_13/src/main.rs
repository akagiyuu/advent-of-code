const INPUT: &str = include_str!("../input.txt");

fn determinant(matrix: [[i64; 2]; 2]) -> i64 {
    matrix[0][0] * matrix[1][1] - matrix[1][0] * matrix[0][1]
}

fn solve(eqs: [[i64; 3]; 2]) -> Option<(u64, u64)> {
    let denom = determinant([[eqs[0][0], eqs[0][1]], [eqs[1][0], eqs[1][1]]]);

    if denom == 0 {
        return None;
    }

    let num_x = determinant([[eqs[0][2], eqs[0][1]], [eqs[1][2], eqs[1][1]]]);
    if num_x % denom != 0 {
        return None;
    }
    let x = num_x / denom;
    if x < 0 {
        return None;
    }

    let num_y = determinant([[eqs[0][0], eqs[0][2]], [eqs[1][0], eqs[1][2]]]);
    if num_y % denom != 0 {
        return None;
    }
    let y = num_y / denom;
    if y < 0 {
        return None;
    }

    Some((x as u64, y as u64))
}

fn parse_line(raw: &str) -> [i64; 2] {
    raw.split(", ")
        .map(|entry| entry[2..].parse::<i64>().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

fn parse(system: &str, offset: i64) -> [[i64; 3]; 2] {
    let mut lines = system.lines();
    let a = parse_line(&lines.next().unwrap()[10..]);
    let b = parse_line(&lines.next().unwrap()[10..]);
    let prize = parse_line(&lines.next().unwrap()[7..]);

    [
        [a[0], b[0], prize[0] + offset],
        [a[1], b[1], prize[1] + offset],
    ]
}

fn calculate_min_token<'a>(systems: impl Iterator<Item = &'a str>, offset: i64) -> u64 {
    systems
        .map(|system| parse(system, offset))
        .filter_map(solve)
        .fold(0, |acc, (s, t)| acc + 3 * s + t)
}

fn main() {
    println!("{}", calculate_min_token(INPUT.split("\n\n"), 0));
    println!("{}", calculate_min_token(INPUT.split("\n\n"), 10000000000000));
}
