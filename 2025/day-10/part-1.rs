use std::{
    collections::{HashSet, VecDeque},
    io::{stdin, BufRead},
};

const NBIT: usize = 64;

fn parse(input: &str) -> (u64, Vec<u64>) {
    let mut parts = input.split(' ');

    let first = parts.next().unwrap();
    let first = &first[1..(first.len() - 1)];
    let mut x = 0u64;
    for (i, c) in first.bytes().enumerate() {
        if c == b'#' {
            x |= 1u64 << i;
        }
    }

    let mut span = vec![];
    for part in parts {
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

    (x, span)
}

fn solve(x: u64, span: Vec<u64>) -> u64 {
    let n = span.len();

    for k in 1..=n {
        let mut m = if k == 0 { 0 } else { (1i64 << k) - 1 };
        while m < (1i64 << n) {
            let mut v = 0;
            for i in 0..n {
                if (m >> i) & 1 == 0 {
                    continue;
                }
                v ^= span[i];
            }
            if v == x {
                return k as u64;
            }

            let c = m & (-m);
            let r = m + c;
            m = (((r ^ m) >> 2) / c) | r
        }
    }

    unreachable!()
}

fn main() {
    let mut res = 0;
    for line in stdin().lock().lines().flatten() {
        let (x, span) = parse(&line);
        res += solve(x, span);
    }
    println!("{res}");
}
