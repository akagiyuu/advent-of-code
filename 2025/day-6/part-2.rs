use std::io::stdin;

enum Operator {
    Addition,
    Multiplication,
}

fn main() {
    let mut grid: Vec<Vec<u8>> = Vec::new();

    while let Some(Ok(line)) = stdin().lines().next() {
        grid.push(line.bytes().collect());
    }

    let n = grid.len();
    let mut starts = Vec::new();
    for (i, &c) in grid[n - 1].iter().enumerate() {
        if c == b'*' || c == b'+' {
            starts.push(i);
        }
    }
    starts.push(grid[n - 1].len() + 1);

    let mut res = 0;

    for window in starts.windows(2) {
        let start = window[0];
        let end = window[1];

        let operator = if grid[n - 1][start] == b'+' {
            Operator::Addition
        } else {
            Operator::Multiplication
        };

        let mut cur = match operator {
            Operator::Addition => 0,
            Operator::Multiplication => 1,
        };

        for i in start..end - 1 {
            let mut x = 0u64;

            let mut j = 0;
            while j < n - 1 && grid[j][i] == b' ' {
                j += 1;
            }

            while j < n - 1 {
                if grid[j][i] == b' ' {
                    break;
                }
                x = 10 * x + (grid[j][i] - b'0') as u64;
                j += 1;
            }
            cur = match operator {
                Operator::Addition => cur + x,
                Operator::Multiplication => cur * x,
            }
        }

        res += cur;
    }

    println!("{res}");
}
