const INPUT: &str = include_str!("input.txt");

#[derive(PartialEq, Eq, Debug)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}
impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Up,
            1 => Self::Right,
            2 => Self::Down,
            3 => Self::Left,
            _ => panic!(),
        }
    }
}
impl Direction {
    fn apply(&self, point: (usize, usize)) -> (usize, usize) {
        match self {
            Direction::Up => (point.0 - 1, point.1),
            Direction::Right => (point.0, point.1 + 1),
            Direction::Down => (point.0 + 1, point.1),
            Direction::Left => (point.0, point.1 - 1),
        }
    }
}

fn calculate(sketch: &[&[u8]], start: (usize, usize)) -> (usize, usize) {
    let row_count = sketch.len();
    let column_count = sketch[0].len();

    let mut number_of_point_in_loop = 0;
    let mut area = 0i64;
    let mut corner = start;

    let mut direction = Direction::Up;
    if start.0 > 0 && [b'|', b'7', b'F'].contains(&sketch[start.0 - 1][start.1]) {
        direction = Direction::Up;
    }
    if start.0 < row_count && [b'|', b'L', b'J'].contains(&sketch[start.0 + 1][start.1]) {
        direction = Direction::Down;
    }
    if start.1 > 0 && [b'-', b'L', b'F'].contains(&sketch[start.0][start.1 - 1]) {
        direction = Direction::Left;
    }
    if start.1 < column_count && [b'-', b'7', b'J'].contains(&sketch[start.0][start.1 + 1]) {
        direction = Direction::Right;
    }
    let mut point = direction.apply(start);
    number_of_point_in_loop += 1;

    loop {
        while sketch[point.0][point.1] == b'-' || sketch[point.0][point.1] == b'|' {
            point = direction.apply(point);
            number_of_point_in_loop += 1;
        }

        // https://en.wikipedia.org/wiki/Shoelace_formula
        area += (corner.0 * point.1) as i64 - (point.0 * corner.1) as i64;
        corner = point;

        direction = match sketch[point.0][point.1] {
            b'F' if direction == Direction::Up => Direction::Right,
            b'F' if direction == Direction::Left => Direction::Down,
            b'J' if direction == Direction::Down => Direction::Left,
            b'J' if direction == Direction::Right => Direction::Up,
            b'7' if direction == Direction::Right => Direction::Down,
            b'7' if direction == Direction::Up => Direction::Left,
            b'L' if direction == Direction::Down => Direction::Right,
            b'L' if direction == Direction::Left => Direction::Up,
            b'S' => break,
            _ => unreachable!(),
        };
        point = direction.apply(point);
        number_of_point_in_loop += 1;
    }

    (
        number_of_point_in_loop / 2,
        (area.unsigned_abs() as usize) / 2 - number_of_point_in_loop / 2 + 1,
    )
}

fn main() {
    let sketch = INPUT
        .trim()
        .lines()
        .map(|l| l.as_bytes())
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
    let (farthest_distance, interior_point) = calculate(&sketch, start);
    println!("{}", farthest_distance);
    println!("{}", interior_point);
}
