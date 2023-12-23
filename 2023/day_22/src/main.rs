use std::collections::{HashSet, VecDeque};

const INPUT: &str = include_str!("input.txt");

type Grid = Vec<Vec<Vec<usize>>>;

#[derive(Debug, Clone)]
struct Coordinate {
    x: usize,
    y: usize,
    z: usize,
}

#[derive(Debug, Clone)]
struct Brick {
    id: usize,
    start: Coordinate,
    end: Coordinate,
    supported_by: HashSet<usize>,
}
impl Brick {
    fn register(&self, grid: &mut Grid) {
        for x in self.start.x..=self.end.x {
            for y in self.start.y..=self.end.y {
                for z in self.start.z..=self.end.z {
                    grid[x][y][z] = self.id;
                }
            }
        }
    }
}

fn parse_bricks(input: &str) -> Vec<Brick> {
    input
        .lines()
        .enumerate()
        .map(|(id, s)| {
            let (start, end) = s.split_once('~').unwrap();
            let start = start
                .split(',')
                .map(|e| e.parse().unwrap())
                .collect::<Vec<usize>>();
            let end = end
                .split(',')
                .map(|e| e.parse().unwrap())
                .collect::<Vec<usize>>();
            Brick {
                id,
                start: Coordinate {
                    x: start[0],
                    y: start[1],
                    z: start[2] - 1,
                },
                end: Coordinate {
                    x: end[0],
                    y: end[1],
                    z: end[2] - 1,
                },
                supported_by: HashSet::new(),
            }
        })
        .collect::<Vec<_>>()
}
fn shift(bricks: &mut [Brick], offset: usize) {
    for brick in bricks {
        let (start_z, end_z) = (brick.start.z, brick.end.z);
        brick.start.z = offset - end_z;
        brick.end.z = offset - start_z;
    }
}
fn project(bricks: &mut [Brick], grid: Grid) {
    let mut grid = grid;
    let length = bricks.len();
    bricks.sort_by(|a, b| b.end.z.cmp(&a.end.z));

    for i in 0..length {
        loop {
            if bricks[i].end.z == grid[0][0].len() - 1 {
                break;
            }
            for x in bricks[i].start.x..=bricks[i].end.x {
                for y in bricks[i].start.y..=bricks[i].end.y {
                    for z in bricks[i].start.z + 1..=bricks[i].end.z + 1 {
                        if grid[x][y][z] == usize::MAX {
                            continue;
                        }
                        bricks[i].supported_by.insert(grid[x][y][z]);
                    }
                }
            }
            if !bricks[i].supported_by.is_empty() {
                break;
            }
            bricks[i].start.z += 1;
            bricks[i].end.z += 1;
        }
        bricks[i].register(&mut grid);
    }
}
fn count_removable_brick(projected_bricks: &[Brick]) -> usize {
    let mut count = 0;
    'outer: for i in 0..projected_bricks.len() {
        let current_id = projected_bricks[i].id;
        for brick in projected_bricks {
            if brick.supported_by.len() == 1 && brick.supported_by.contains(&current_id) {
                continue 'outer;
            }
        }
        count += 1;
    }
    count
}

fn sum_fall_bricks(projected_bricks: Vec<Brick>) -> usize {
    let brick_count = projected_bricks.len();
    let mut count = 0;
    for current_brick in 0..brick_count {
        let mut bricks = projected_bricks.clone();
        let mut is_removed = vec![false; brick_count];
        let start_id = bricks[current_brick].id;

        is_removed[start_id] = true;
        let mut falling_bricks = VecDeque::from([start_id]);

        while let Some(current_id) = falling_bricks.pop_front() {
            for brick in bricks.iter_mut() {
                if !brick.supported_by.contains(&current_id) {
                    continue;
                }
                brick.supported_by.remove(&current_id);
                if !brick.supported_by.is_empty() {
                    continue;
                }
                let next_id = brick.id;
                if !is_removed[next_id] {
                    falling_bricks.push_back(next_id);
                    is_removed[next_id] = true;
                    count += 1;
                }
            }
        }
    }
    count
}

fn main() {
    let mut bricks = parse_bricks(INPUT);

    let max_x = bricks.iter().map(|b| b.end.x).max().unwrap();
    let max_y = bricks.iter().map(|b| b.end.y).max().unwrap();
    let max_z = bricks.iter().map(|b| b.end.z).max().unwrap();
    let grid = vec![vec![vec![usize::MAX; max_z + 1]; max_y + 1]; max_x + 1];

    shift(&mut bricks, max_z);
    project(&mut bricks, grid);

    println!("{}", count_removable_brick(&bricks));
    println!("{}", sum_fall_bricks(bricks));
}
