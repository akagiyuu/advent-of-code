use std::cmp::Ordering;

const INPUT: &str = include_str!("../input.txt");

fn parse_location(raw: &str) -> (i64, i64) {
    let (x, y) = raw.split_once(',').unwrap();
    let x = x.parse().unwrap();
    let y = y.parse().unwrap();

    (x, y)
}

fn parse_robot(raw: &str) -> ((i64, i64), (i64, i64)) {
    let (p, v) = raw.split_once(' ').unwrap();
    let p = parse_location(p[2..].trim());
    let v = parse_location(v[2..].trim());

    (p, v)
}

fn process(p: (i64, i64), v: (i64, i64), iter: i64, xs: i64, ys: i64) -> (i64, i64) {
    let x = p.0 + v.0 * iter;
    let x = (x % xs + xs) % xs;

    let y = p.1 + v.1 * iter;
    let y = (y % ys + ys) % ys;

    (x, y)
}

fn calculate_safety_factor(robots: &str, iter: i64, xs: i64, ys: i64) -> usize {
    let mut robot_count = [0; 4];
    let coords = robots
        .lines()
        .map(parse_robot)
        .map(|(p, v)| process(p, v, iter, xs, ys));

    for (x, y) in coords {
        match (x.cmp(&(xs / 2)), y.cmp(&(ys / 2))) {
            (Ordering::Less, Ordering::Less) => robot_count[0] += 1,
            (Ordering::Less, Ordering::Greater) => robot_count[1] += 1,
            (Ordering::Greater, Ordering::Less) => robot_count[2] += 1,
            (Ordering::Greater, Ordering::Greater) => robot_count[3] += 1,
            _ => {}
        }
    }

    robot_count.iter().product()
}

fn main() {
    println!("{}", calculate_safety_factor(INPUT, 100, 101, 103))
}
