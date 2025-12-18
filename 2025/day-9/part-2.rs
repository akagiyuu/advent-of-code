use std::{io::stdin, mem::swap, ops::Sub};

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

fn polygon_contain(p: Point, polygon: &[Point]) -> bool {
    let n = polygon.len();

    let mut inside = false;

    for i in 0..n {
        let j = (i + 1) % n;
        let a = &polygon[i];
        let b = &polygon[j];

        let range_x = i64::min(a.x, b.x)..=i64::max(a.x, b.x);
        let range_y = i64::min(a.y, b.y)..=i64::max(a.y, b.y);

        if range_x.contains(&p.x) && range_y.contains(&p.y) {
            return true;
        }

        if a.x == b.x && (a.y > p.y) != (b.y > p.y) && a.x > p.x {
            inside = !inside;
        }
    }

    inside
}

fn solve(polygon: &[Point]) -> u64 {
    let n = polygon.len();
    let mut res = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            let Point { x: x1, y: y1 } = polygon[i];
            let Point { x: x2, y: y2 } = polygon[j];

            let mut contain = true;
            for x in [x1, x2] {
                for y in [y1, y2] {
                    contain &= polygon_contain(Point { x, y }, polygon);
                }
            }
            if !contain {
                continue;
            }

            res = res.max((x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1));
        }
    }

    res
}

fn main() {
    let polygon: Vec<_> = stdin()
        .lines()
        .filter_map(|line| match line {
            Ok(line) => {
                let (x, y) = line.split_once(',').unwrap();
                let x = x.parse().unwrap();
                let y = y.parse().unwrap();

                Some(Point { x, y })
            }
            _ => None,
        })
        .collect();
    println!("{}", solve(&polygon));
}
