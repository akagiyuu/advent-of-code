use std::{
    collections::HashMap,
    io::{BufRead, stdin},
};

const SERVER: &str = "svr";
const FFT: &str = "fft";
const DAC: &str = "dac";
const OUT: &str = "out";

fn encode(u: &str, encoder: &mut HashMap<String, usize>) -> usize {
    match encoder.get(u) {
        Some(&vertex) => vertex,
        None => {
            let vertex = encoder.len();
            encoder.insert(u.to_string(), vertex);

            vertex
        }
    }
}

fn parse(input: impl Iterator<Item = String>) -> (Vec<Vec<usize>>, HashMap<String, usize>) {
    let mut encoder = HashMap::new();
    let mut adj = vec![];

    for line in input {
        let (u, vs) = line.split_once(':').unwrap();
        let u = encode(u, &mut encoder);
        adj.resize(adj.len().max(encoder.len()), vec![]);

        for v in vs.trim().split(' ') {
            let v = encode(v, &mut encoder);
            adj.resize(adj.len().max(encoder.len()), vec![]);
            adj[u].push(v);
        }
    }

    (adj, encoder)
}

fn count_way(u: usize, end: usize, cnt: &mut Vec<usize>, adj: &[Vec<usize>]) -> usize {
    if cnt[u] < usize::MAX {
        return cnt[u];
    }

    let mut res = 0;
    for &v in &adj[u] {
        res += count_way(v, end, cnt, adj);
    }

    cnt[u] = res;
    res
}

fn solve(adj: &[Vec<usize>], encoder: &HashMap<String, usize>) -> usize {
    let mut cnt = vec![0; adj.len()];
    let server = encoder[SERVER];
    let fft = encoder[FFT];
    let dac = encoder[DAC];
    let out = encoder[OUT];

    let first = [server, fft, dac, out]
        .windows(2)
        .map(|x| {
            let start = x[0];
            let end = x[1];
            cnt.fill(usize::MAX);
            cnt[end] = 1;
            count_way(start, end, &mut cnt, adj)
        })
        .product::<usize>();

    let second = [server, dac, fft, out]
        .windows(2)
        .map(|x| {
            let start = x[0];
            let end = x[1];
            cnt.fill(usize::MAX);
            cnt[end] = 1;
            count_way(start, end, &mut cnt, adj)
        })
        .product::<usize>();

    first + second
}

fn main() {
    let (adj, encoder) = parse(stdin().lock().lines().flatten());
    println!("{}", solve(&adj, &encoder));
}
