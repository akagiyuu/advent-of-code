use std::{
    collections::HashMap,
    io::{stdin, BufRead},
};

const INF: u64 = 10u64.pow(9);

fn parse(input: &str) -> (Vec<u64>, Vec<u64>) {
    let mut parts = input.split(' ');
    parts.next().unwrap();

    let mut span = vec![];
    let mut prev = "";
    for part in parts {
        prev = part;
        if part.as_bytes()[0] == b'{' {
            break;
        }
        let part = &part[1..(part.len() - 1)];
        let mut x = 0;
        for i in part.split(',') {
            let i = i.parse::<u32>().unwrap();
            x |= 1u64 << i;
        }
        span.push(x);
    }
    let part = &prev[1..(prev.len() - 1)];
    let mut x = vec![];
    for i in part.split(',') {
        let i = i.parse::<u64>().unwrap();
        x.push(i);
    }

    (x, span)
}

fn solve(x: Vec<u64>, span: &[u64], cache: &mut HashMap<Vec<u64>, u64>) -> u64 {
    match cache.get(&x) {
        Some(&res) => return res,
        None => {}
    }

    let n = span.len();
    let sz = x.len();
    if x.iter().all(|&x| x == 0) {
        return 0;
    }

    let mut x_bit = 0;
    for (i, &parity) in x.iter().enumerate() {
        if parity & 1 == 1 {
            x_bit |= 1u64 << i;
        }
    }

    let mut res = INF;
    'outer: for m in 0..(1u64 << n) {
        let mut v = 0;
        for i in 0..n {
            if (m >> i) & 1 == 0 {
                continue;
            }
            v ^= span[i];
        }
        if v != x_bit {
            continue;
        }

        let mut x = x.clone();
        for i in 0..n {
            if (m >> i) & 1 == 0 {
                continue;
            }
            for j in 0..sz {
                let cur = (span[i] >> j) & 1;
                if x[j] < cur {
                    continue 'outer;
                }
                x[j] -= cur;
            }
        }
        for j in 0..sz {
            x[j] >>= 1;
        }
        res = res.min(m.count_ones() as u64 + 2 * solve(x, span, cache));
    }

    cache.insert(x.clone(), res);

    res
}

fn main() {
    let mut res = 0;
    for line in stdin().lock().lines().flatten() {
        let (x, span) = parse(&line);
        let mut cache = HashMap::new();
        res += solve(x, &span, &mut cache);
    }
    println!("{res}");
}
