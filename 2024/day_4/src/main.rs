const INPUT: &str = include_str!("../input.txt");

const XMAS: &[u8] = b"XMAS";

fn _count_xmas(i: usize, j: usize, grid: &[Vec<u8>]) -> usize {
    if grid[i][j] != XMAS[0] {
        return 0;
    }

    let row_count = grid.len();
    let column_count = grid[0].len();

    let mut count = 0;

    for dx in [-1, 0, 1] {
        for dy in [-1, 0, 1] {
            if dx == 0 && dy == 0 {
                continue;
            }

            let mut i = i as isize + dx;
            let mut j = j as isize + dy;
            let mut satisfy = true;
            for step in 1..4 {
                if !(0..row_count as isize).contains(&i)
                    || !(0..column_count as isize).contains(&j)
                    || grid[i as usize][j as usize] != XMAS[step]
                {
                    satisfy = false;
                    break;
                }

                i += dx;
                j += dy;
            }

            if satisfy {
                count += 1;
            }
        }
    }

    count
}

fn count_xmas(grid: &[Vec<u8>]) -> usize {
    let mut res = 0;

    let row_count = grid.len();
    let column_count = grid[0].len();

    for i in 0..row_count {
        for j in 0..column_count {
            res += _count_xmas(i, j, grid);
        }
    }

    res
}

fn count_mas(grid: &[Vec<u8>]) -> usize {
    let row_count = grid.len();
    let column_count = grid[0].len();

    let mut count = 0;

    for i in 1..row_count - 1 {
        for j in 1..column_count - 1 {
            if grid[i][j] != b'A' {
                continue;
            }

            let left_diagonal = &[grid[i - 1][j - 1], grid[i][j], grid[i + 1][j + 1]];
            let right_diagonal = &[grid[i - 1][j + 1], grid[i][j], grid[i + 1][j - 1]];

            match (left_diagonal, right_diagonal) {
                (b"MAS", b"MAS") | (b"SAM", b"MAS") | (b"MAS", b"SAM") | (b"SAM", b"SAM") => {
                    count += 1
                }
                _ => {}
            }
        }
    }

    count
}

fn main() {
    let grid: Vec<Vec<_>> = INPUT.lines().map(|line| line.bytes().collect()).collect();

    println!("{}", count_xmas(&grid));
    println!("{}", count_mas(&grid));
}
