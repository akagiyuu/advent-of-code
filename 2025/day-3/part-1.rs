use std::io::stdin;

const N: usize = 9;

fn solve(banks: &str) -> u64 {
    let mut nearest: [Option<usize>; N] = [None; N];
    let mut farthest: [Option<usize>; N] = [None; N];

    for (i, battery) in banks.as_bytes().iter().enumerate() {
        let battery = (battery - b'1') as usize;

        nearest[battery] = match nearest[battery] {
            None => Some(i),
            Some(j) => Some(j.min(i)),
        };
        farthest[battery] = match farthest[battery] {
            None => Some(i),
            Some(j) => Some(j.max(i)),
        }
    }

    for a in (1..=9).rev() {
        for b in (1..=9).rev() {
            let Some(i) = nearest[a - 1] else {
                continue;
            };
            let Some(j) = farthest[b - 1] else {
                continue;
            };
            if i < j {
                return a as u64 * 10 + b as u64;
            }
        }
    }

    unreachable!()
}

fn main() {
    let mut res = 0;

    while let Some(Ok(banks)) = stdin().lines().next() {
        res += solve(&banks);
    }

    println!("{res}");
}
