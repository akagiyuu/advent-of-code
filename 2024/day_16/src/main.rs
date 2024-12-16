use std::collections::{BinaryHeap, HashSet};

const INPUT: &str = include_str!("../input.txt");

fn find(c: u8, grid: &[Vec<u8>]) -> (usize, usize) {
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == c {
                return (i, j);
            }
        }
    }

    unreachable!()
}

#[derive(PartialEq, Eq)]
struct State {
    pos: (usize, usize),
    dir: usize,
    score: usize,
    histories: Vec<(usize, usize)>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn apply(pos: (usize, usize), dir: usize) -> (usize, usize) {
    match dir {
        0 => (pos.0 - 1, pos.1),
        1 => (pos.0, pos.1 + 1),
        2 => (pos.0 + 1, pos.1),
        3 => (pos.0, pos.1 - 1),
        _ => unreachable!(),
    }
}

fn get_metrics(grid: &[Vec<u8>]) -> (usize, usize) {
    let start = find(b'S', grid);
    let end = find(b'E', grid);

    let mut is_visited = vec![vec![[false; 4]; grid[0].len()]; grid.len()];
    let mut min_score = None;

    let mut queue = BinaryHeap::new();
    queue.push(State {
        pos: start,
        dir: 1,
        score: 0,
        histories: vec![start],
    });

    let mut nodes = HashSet::new();

    while let Some(State {
        pos,
        dir,
        score,
        histories,
    }) = queue.pop()
    {
        is_visited[pos.0][pos.1][dir] = true;

        if pos == end {
            match min_score {
                Some(min_score) if min_score == score => {
                    for &node in &histories {
                        nodes.insert(node);
                    }
                }
                None => {
                    min_score = Some(score);
                    for &node in &histories {
                        nodes.insert(node);
                    }
                }
                _ => {}
            }
        }

        for delta_dir in [0, 1, 3] {
            let next_dir = (dir + delta_dir) % 4;
            let next_pos = apply(pos, next_dir);
            if grid[next_pos.0][next_pos.1] == b'#' || is_visited[next_pos.0][next_pos.1][next_dir]
            {
                continue;
            }

            let next_score = score + if delta_dir == 0 { 0 } else { 1000 } + 1;
            let mut next_histories = histories.clone();
            next_histories.push(next_pos);

            queue.push(State {
                pos: next_pos,
                dir: next_dir,
                score: next_score,
                histories: next_histories,
            });
        }
    }

    (min_score.unwrap_or(usize::MAX), nodes.len())
}

fn main() {
    let grid: Vec<_> = INPUT.lines().map(|line| line.as_bytes().to_vec()).collect();

    println!("{:?}", get_metrics(&grid));
}
