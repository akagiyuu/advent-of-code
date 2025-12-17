use std::{
    collections::{BTreeMap, BTreeSet, HashSet},
    io::stdin,
    mem::swap,
};

#[derive(Debug)]
struct Point {
    pub x: u64,
    pub y: u64,
    pub z: u64,
}

impl Point {
    pub fn squared_distance(&self, other: &Point) -> u64 {
        let dx = self.x.abs_diff(other.x);
        let dy = self.y.abs_diff(other.y);
        let dz = self.z.abs_diff(other.z);

        dx * dx + dy * dy + dz * dz
    }
}

struct DSU {
    pub parent: Vec<usize>,
    pub sz: Vec<u64>,
    pub comps: usize,
}

impl DSU {
    pub fn new(n: usize) -> DSU {
        let mut parent = vec![0; n + 1];
        for i in 1..=n {
            parent[i] = i;
        }

        DSU {
            parent,
            sz: vec![1; n + 1],
            comps: n,
        }
    }

    pub fn find(&self, mut x: usize) -> usize {
        while self.parent[x] != x {
            x = self.parent[x];
        }

        x
    }

    pub fn unite(&mut self, mut a: usize, mut b: usize) {
        a = self.find(a);
        b = self.find(b);
        if a == b {
            return;
        }

        if self.sz[a] > self.sz[b] {
            swap(&mut a, &mut b);
        }
        self.parent[a] = b;
        self.sz[b] += self.sz[a];
        self.comps -= 1;
    }
}

fn solve(points: &[Point]) -> u64 {
    let n = points.len();
    let mut dist = Vec::with_capacity(n * (n - 1) / 2);
    for u in 0..n {
        for v in (u + 1)..n {
            dist.push((points[u].squared_distance(&points[v]), u + 1, v + 1));
        }
    }
    dist.sort_by_key(|(x, _, _)| *x);

    let mut dsu = DSU::new(n);
    let mut i = 0;
    while dsu.comps > 1 {
        let (_, u, v) = dist[i];
        dsu.unite(u, v);
        i += 1;
    }
    i -= 1;

    let (_, u, v) = dist[i];

    points[u - 1].x * points[v - 1].x
}

fn main() {
    let mut points = vec![];

    while let Some(Ok(line)) = stdin().lines().next() {
        let raw: Vec<_> = line.split(',').map(|x| x.parse::<u64>().unwrap()).collect();

        points.push(Point {
            x: raw[0],
            y: raw[1],
            z: raw[2],
        });
    }

    println!("{}", solve(&points));
}
