use std::io::stdin;

const MOD: u64 = 100;

const fn next(a: u64) -> u64 {
    if a == MOD - 1 {
        0
    } else {
        a + 1
    }
}

const fn prev(a: u64) -> u64 {
    if a == 0 {
        MOD - 1
    } else {
        a - 1
    }
}

fn main() {
    let mut cur = 50;
    let mut res = 0;

    while let Some(Ok(line)) = stdin().lines().next() {
        let direction = line.as_bytes()[0];
        let cnt: u64 = line.as_str()[1..].parse().unwrap();

        match direction {
            b'L' => {
                for _ in 0..cnt {
                    cur = next(cur);
                    if cur == 0 {
                        res += 1;
                    }
                }
            }
            b'R' => {
                for _ in 0..cnt {
                    cur = prev(cur);
                    if cur == 0 {
                        res += 1;
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    println!("{res}");
}
