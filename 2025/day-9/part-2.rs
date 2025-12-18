use std::{
    collections::{BTreeSet, HashMap},
    io::stdin,
    mem::swap,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

fn area(a: Point, b: Point) -> usize {
    (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1)
}

fn build(
    polygon: &[Point],
) -> (
    impl Fn(Point) -> Point,
    impl Fn(Point) -> Point,
    usize,
    usize,
) {
    let mut xs = BTreeSet::new();
    let mut ys = BTreeSet::new();
    for &Point { x, y } in polygon {
        xs.insert(x);
        ys.insert(y);
    }
    let xs: Vec<_> = xs.into_iter().collect();
    let ys: Vec<_> = ys.into_iter().collect();

    let mut map_x = HashMap::<usize, usize>::with_capacity(xs.len());
    for (i, &x) in xs.iter().enumerate() {
        map_x.insert(x, i);
    }

    let mut map_y = HashMap::<usize, usize>::with_capacity(ys.len());
    for (i, &y) in ys.iter().enumerate() {
        map_y.insert(y, i);
    }

    let sz_x = xs.len();
    let sz_y = ys.len();

    (
        move |p| Point {
            x: map_x[&p.x],
            y: map_y[&p.y],
        },
        move |p| Point {
            x: xs[p.x as usize],
            y: ys[p.y as usize],
        },
        sz_x,
        sz_y,
    )
}

fn polygon_contain(p: Point, polygon: &[Point]) -> bool {
    let n = polygon.len();

    let mut inside = false;

    for i in 0..n {
        let j = (i + 1) % n;
        let a = &polygon[i];
        let b = &polygon[j];

        if a.x == b.x && (a.y > p.y) != (b.y > p.y) && a.x > p.x {
            inside = !inside;
        }
    }

    inside
}

fn solve(mut polygon: Vec<Point>) -> usize {
    let n = polygon.len();

    let (encoder, decoder, sz_x, sz_y) = build(&polygon);
    for i in 0..n {
        polygon[i] = encoder(polygon[i]);
    }

    let mut grid = vec![vec![0; sz_y]; sz_x];
    for i in 0..n {
        let j = (i + 1) % n;
        let Point {
            x: mut x1,
            y: mut y1,
        } = polygon[i];
        let Point {
            x: mut x2,
            y: mut y2,
        } = polygon[j];
        if x1 > x2 {
            swap(&mut x1, &mut x2);
        }
        if y1 > y2 {
            swap(&mut y1, &mut y2);
        }

        for x in x1..=x2 {
            for y in y1..=y2 {
                grid[x][y] = 1;
            }
        }
    }
    for x in 0..sz_x {
        for y in 0..sz_y {
            if grid[x][y] == 1 || !polygon_contain(Point { x, y }, &polygon) {
                continue;
            }

            grid[x][y] = 1;
        }
    }
    for x in 1..sz_x {
        grid[x][0] += grid[x - 1][0];
    }
    for y in 1..sz_y {
        grid[0][y] += grid[0][y - 1];
    }
    for x in 1..sz_x {
        for y in 1..sz_y {
            grid[x][y] += grid[x - 1][y] + grid[x][y - 1] - grid[x - 1][y - 1];
        }
    }

    let get_sum = move |x1: usize, x2: usize, y1: usize, y2: usize| {
        let mut res = grid[x2][y2];
        if x1 >= 1 && y1 >= 1 {
            res += grid[x1 - 1][y1 - 1];
        }
        if x1 >= 1 {
            res -= grid[x1 - 1][y2];
        }
        if y1 >= 1 {
            res -= grid[x2][y1 - 1];
        }

        res
    };

    let mut res = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            let Point {
                x: mut x1,
                y: mut y1,
            } = polygon[i];
            let Point {
                x: mut x2,
                y: mut y2,
            } = polygon[j];
            if x1 > x2 {
                swap(&mut x1, &mut x2);
            }
            if y1 > y2 {
                swap(&mut y1, &mut y2);
            }
            if get_sum(x1, x2, y1, y2) != area(polygon[i], polygon[j]) {
                continue;
            }
            res = res.max(area(decoder(polygon[i]), decoder(polygon[j])));
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
    println!("{}", solve(polygon));
}
