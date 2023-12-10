use std::collections::VecDeque;

const INPUT: &str = include_str!("input.txt");

const PIPES: [u8; 7] = [b'|', b'-', b'L', b'J', b'7', b'F', b'S'];

#[derive(PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
impl Direction {
    fn opposite(self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }
}

fn get_pipe_directions(pipe: u8) -> Vec<Direction> {
    match pipe {
        b'|' => vec![Direction::Up, Direction::Down],
        b'-' => vec![Direction::Left, Direction::Right],
        b'L' => vec![Direction::Up, Direction::Right],
        b'J' => vec![Direction::Up, Direction::Left],
        b'7' => vec![Direction::Down, Direction::Left],
        b'F' => vec![Direction::Down, Direction::Right],
        b'S' => vec![
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ],
        _ => vec![],
    }
}

fn is_movable(direction: Direction, next_pipe_directions: Vec<Direction>) -> bool {
    next_pipe_directions.contains(&direction.opposite())
}

type Sketch = Vec<Vec<u8>>;
type Graph = Vec<Vec<Vec<(usize, usize)>>>;

fn print_sketch(sketch: &Sketch) {
    for row in sketch {
        println!("{}", String::from_utf8_lossy(row));
    }
}

fn get_farthest_distance(sketch: &Sketch, start: (usize, usize)) -> usize {
    let mut distances = vec![vec![usize::MAX; sketch[0].len()]; sketch.len()];
    distances[start.0][start.1] = 0;

    let mut queue = VecDeque::from([start]);

    let mut is_visited = vec![vec![false; sketch[0].len()]; sketch.len()];
    is_visited[start.0][start.1] = true;

    let mut max_distance = 0;

    while let Some((i, j)) = queue.pop_front() {
        for direction in get_pipe_directions(sketch[i][j]) {
            let mut next = (i, j);
            if direction == Direction::Up && i > 0 {
                next.0 -= 1;
            }
            if direction == Direction::Down && i < sketch.len() - 1 {
                next.0 += 1;
            }
            if direction == Direction::Left && j > 0 {
                next.1 -= 1;
            }
            if direction == Direction::Right && j < sketch[0].len() - 1 {
                next.1 += 1;
            }
            if !is_movable(direction, get_pipe_directions(sketch[next.0][next.1])) {
                continue;
            }
            if is_visited[next.0][next.1] {
                continue;
            }
            queue.push_back(next);
            is_visited[next.0][next.1] = true;
            distances[next.0][next.1] = distances[i][j] + 1;
            max_distance = max_distance.max(distances[next.0][next.1]);
        }
    }

    max_distance
}

fn generate_reduced_sketch_and_graph(sketch: &Sketch, start: (usize, usize)) -> (Sketch, Graph) {
    let mut queue = VecDeque::from([start]);

    let mut is_visited = vec![vec![false; sketch[0].len()]; sketch.len()];
    is_visited[start.0][start.1] = true;

    let mut graph = vec![vec![vec![]; sketch[0].len()]; sketch.len()];

    while let Some((i, j)) = queue.pop_front() {
        for direction in get_pipe_directions(sketch[i][j]) {
            let mut next = (i, j);
            if direction == Direction::Up && i > 0 {
                next.0 -= 1;
            }
            if direction == Direction::Down && i < sketch.len() - 1 {
                next.0 += 1;
            }
            if direction == Direction::Left && j > 0 {
                next.1 -= 1;
            }
            if direction == Direction::Right && j < sketch[0].len() - 1 {
                next.1 += 1;
            }
            if !is_movable(direction, get_pipe_directions(sketch[next.0][next.1])) {
                continue;
            }
            graph[i][j].push(next);
            if is_visited[next.0][next.1] {
                continue;
            }
            queue.push_back(next);
            is_visited[next.0][next.1] = true;
        }
    }

    let mut reduced_sketch = vec![vec![b'.'; sketch[0].len()]; sketch.len()];
    for i in 0..sketch.len() {
        for j in 0..sketch[0].len() {
            if is_visited[i][j] {
                reduced_sketch[i][j] = sketch[i][j];
            }
        }
    }

    (reduced_sketch, graph)
}

fn get_connection_sketch(reduced_sketch: &[Vec<u8>]) -> Vec<Vec<Direction>> {
    todo!()
}

fn count_enclosed_tile(reduced_sketch: &Sketch, graph: &Graph) -> usize {
    let mut sketch = reduced_sketch.clone();
    let row_count = sketch.len();
    let column_count = sketch[0].len();

    for i in 0..row_count {
        for j in 0..column_count {
            if sketch[i][j] != b'.' {
                continue;
            }
            if i > 0 && PIPES.contains(&sketch[i - 1][j]) {
                if sketch[i - 1][j] != b'-' {
                    sketch[i][j] = b'O';
                    continue;
                }
            }
            if i < row_count - 1 && PIPES.contains(&sketch[i + 1][j]) {
                if sketch[i + 1][j] != b'-' {
                    sketch[i][j] = b'O';
                    continue;
                }
            }
            if j > 0 && PIPES.contains(&sketch[i][j - 1]) {
                if sketch[i][j - 1] != b'|' {
                    sketch[i][j] = b'O';
                    continue;
                }
            }
            if j < column_count - 1 && PIPES.contains(&sketch[i][j + 1]) {
                if sketch[i][j + 1] != b'|' {
                    sketch[i][j] = b'O';
                    continue;
                }
            }
        }
    }
    print_sketch(&sketch);
    0
}

fn main() {
    let sketch = INPUT
        .lines()
        .map(|l| l.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut start = (0, 0);
    for i in 0..sketch.len() {
        for j in 0..sketch[0].len() {
            if sketch[i][j] == b'S' {
                start = (i, j);
                break;
            }
        }
    }
    let farthest_distance = get_farthest_distance(&sketch, start);
    println!("{}", farthest_distance);
    // let (reduced_sketch, graph) = generate_reduced_sketch_and_graph(&sketch, start);
}
