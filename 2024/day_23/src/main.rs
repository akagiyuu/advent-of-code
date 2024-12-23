#![feature(iter_intersperse)]

use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

const INPUT: &str = include_str!("../input.txt");

type Encoder<'a> = HashMap<&'a [u8], usize>;
type Decoder<'a> = Vec<&'a [u8]>;

fn upper_bound<T: Ord>(a: &[T], x: &T) -> usize {
    a.binary_search_by(|element| match element.cmp(x) {
        Ordering::Equal => Ordering::Less,
        ord => ord,
    })
    .unwrap_err()
}

fn encode<'a>(computer: &'a [u8], encoder: &mut Encoder<'a>, decoder: &mut Decoder<'a>) -> usize {
    let length = encoder.len();

    match encoder.get(computer) {
        Some(&v) => v,
        None => {
            encoder.insert(computer, length);
            decoder.push(computer);
            length
        }
    }
}

fn decode<'a>(i: usize, decoder: &Decoder<'a>) -> &'a [u8] {
    decoder[i]
}

fn build_graph<'a>(raw: &'static str) -> (Vec<HashSet<usize>>, Decoder<'a>, Encoder<'a>) {
    let mut graph = vec![];
    let mut encoder = HashMap::new();
    let mut decoder = vec![];

    for line in raw.lines() {
        let (a, b) = line.split_once('-').unwrap();
        let i = encode(a.as_bytes(), &mut encoder, &mut decoder);
        let j = encode(b.as_bytes(), &mut encoder, &mut decoder);
        while graph.len() <= i.max(j) {
            graph.push(HashSet::new());
        }
        graph[i].insert(j);
        graph[j].insert(i);
    }

    (graph, decoder, encoder)
}

fn count_group(graph: &[HashSet<usize>], decoder: &Decoder<'_>) -> usize {
    let n = graph.len();

    let mut count = 0;

    for i in 0..n {
        for &j in &graph[i] {
            if i >= j {
                continue;
            }
            for &k in &graph[j] {
                if j >= k {
                    continue;
                }
                if !graph[i].contains(&k) {
                    continue;
                }
                if [i, j, k].into_iter().any(|x| decode(x, decoder)[0] == b't') {
                    count += 1;
                }
            }
        }
    }

    count
}

fn bron_kerbosch(
    r: HashSet<usize>,
    p: HashSet<usize>,
    x: HashSet<usize>,
    graph: &Vec<HashSet<usize>>,
    cliques: &mut Vec<HashSet<usize>>,
) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r);
        return;
    }

    let mut p_clone = p.clone();
    for v in p {
        let mut r_new = r.clone();
        r_new.insert(v);

        let neighbors = &graph[v];
        let p_new: HashSet<_> = p_clone.intersection(neighbors).cloned().collect();
        let x_new: HashSet<_> = x.intersection(neighbors).cloned().collect();

        bron_kerbosch(r_new, p_new, x_new, graph, cliques);

        p_clone.remove(&v);
        let mut x_new = x.clone();
        x_new.insert(v);
    }
}

fn main() {
    let (graph, decoder, encoder) = build_graph(INPUT);

    println!("{}", count_group(&graph, &decoder));

    let mut cliques = Vec::new();
    bron_kerbosch(
        HashSet::new(),
        HashSet::from_iter(0..graph.len()),
        HashSet::new(),
        &graph,
        &mut cliques,
    );

    let mut password: Vec<_> = cliques
        .into_iter()
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap()
        .into_iter()
        .map(|x| String::from_utf8(decode(x, &decoder).to_vec()).unwrap())
        .collect();
    password.sort();

    println!("{}", password.join(","));
}
