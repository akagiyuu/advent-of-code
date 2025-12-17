use std::io::stdin;

const K: usize = 12;

fn solve(banks: &str) -> u64 {
    let n = banks.len();
    let mut dp = vec![vec![0; n]; K + 1];

    for (i, battery) in banks.bytes().enumerate().rev() {
        let battery = (battery - b'0') as u64;
        dp[1][i] = battery;
        if i < n - 1 {
            dp[1][i] = u64::max(dp[1][i], dp[1][i + 1]);
        }
    }

    for k in 2..=K {
        let scale = 10u64.pow(k as u32 - 1);
        for i in (0..=n - k).rev() {
            let battery = (banks.as_bytes()[i] - b'0') as u64;
            dp[k][i] = u64::max(dp[k][i + 1], battery * scale + dp[k - 1][i + 1]);
        }
    }

    dp[K][0]
}

fn main() {
    let mut res = 0;

    while let Some(Ok(banks)) = stdin().lines().next() {
        res += solve(&banks);
    }

    println!("{res}");
}
