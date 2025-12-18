use std::io::stdin;

fn solve(points: &[(u64, u64)]) -> u64 {
    let n = points.len();
    let mut res = 0;

    for i in 0..n {
        let (x1, y1) = points[i];
        for j in (i + 1)..n {
            let (x2, y2) = points[j];

            res = res.max((x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1));
        }
    }

    res
}

fn main() {
    let points: Vec<_> = stdin()
        .lines()
        .filter_map(|line| match line {
            Ok(line) => {
                let (x, y) = line.split_once(',').unwrap();
                let x = x.parse::<u64>().unwrap();
                let y = y.parse::<u64>().unwrap();

                Some((x, y))
            }
            _ => None,
        })
        .collect();

    println!("{}", solve(&points));
}
