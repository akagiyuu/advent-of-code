use std::io::{stdin, BufRead, Read};

fn parse(input: &str) -> Vec<(u64, u64, u64)> {
    let parts = input.split("\n\n");
    let regions = parts.last().unwrap();

    regions
        .lines()
        .map(|line| {
            let (rectangle, cnt) = line.split_once(':').unwrap();

            let (x, y) = rectangle.split_once('x').unwrap();
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();

            let cnt = cnt
                .trim()
                .split(' ')
                .map(|x| x.parse::<u64>().unwrap())
                .sum();

            (x, y, cnt)
        })
        .collect()
}

fn solve(regions: &[(u64, u64, u64)]) -> u64 {
    let mut res = 0;

    for &(x, y, cnt) in regions {
        if x * y >= cnt * 8 {
            res += 1;
        }
    }

    res
}

// Why this work?
fn main() {
    let mut buffer = String::new();
    stdin().lock().read_to_string(&mut buffer);
    let regions = parse(&buffer);
    println!("{}", solve(&regions));
}
