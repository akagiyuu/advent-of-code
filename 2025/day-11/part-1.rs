use std::{
    collections::HashMap,
    io::{stdin, BufRead},
};

const START: &str = "you";
const END: &str = "out";

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

fn dfs(u: usize, end: usize, visitied: &mut Vec<bool>, adj: &[Vec<usize>]) -> usize {
    if u == end {
        return 1;
    }

    let mut res = 0;
    for &v in &adj[u] {
        if visitied[v] {
            continue;
        }
        visitied[v] = true;
        res += dfs(v, end, visitied, adj);
        visitied[v] = false;
    }

    return res;
}

fn solve(adj: &[Vec<usize>], encoder: &HashMap<String, usize>) -> usize {
    let start = encoder[START];
    let end = encoder[END];

    let mut visited = vec![false; adj.len()];

    dfs(start, end, &mut visited, adj)
}

fn main() {
    let (adj, encoder) = parse(stdin().lock().lines().flatten());
    println!("{}", solve(&adj, &encoder));
}
