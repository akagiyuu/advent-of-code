use std::ops::Range;

pub type Coordinate = (usize, usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}
impl From<u8> for Direction {
    fn from(value: u8) -> Direction {
        match value {
            b'R' | b'0' => Direction::Right,
            b'D' | b'1' => Direction::Down,
            b'L' | b'2' => Direction::Left,
            b'U' | b'3' => Direction::Up,
            _ => unreachable!(),
        }
    }
}
impl Direction {
    pub fn unchecked_apply(&self, coordinate: Coordinate, count: usize) -> Coordinate {
        assert!(count > 0);
        match self {
            Direction::Up => (coordinate.0 - count, coordinate.1),
            Direction::Left => (coordinate.0, coordinate.1 - count),
            Direction::Down => (coordinate.0 + count, coordinate.1),
            Direction::Right => (coordinate.0, coordinate.1 + count),
        }
    }
    pub fn apply(
        &self,
        coordinate: Coordinate,
        count: usize,
        row_boundary: Range<usize>,
        column_boundary: Range<usize>,
    ) -> Option<Coordinate> {
        assert!(count > 0);
        match self {
            Direction::Up if coordinate.0 >= row_boundary.start + count => {
                Some((coordinate.0 - count, coordinate.1))
            }
            Direction::Down if coordinate.0 + count < row_boundary.end => {
                Some((coordinate.0 + count, coordinate.1))
            }
            Direction::Left if coordinate.1 >= column_boundary.start + count => {
                Some((coordinate.0, coordinate.1 - count))
            }
            Direction::Right if coordinate.1 + count < column_boundary.end => {
                Some((coordinate.0, coordinate.1 + count))
            }
            _ => None,
        }
    }
}

pub fn intersect<T: Ord + Copy>(a: Range<T>, b: Range<T>) -> Range<T> {
    (a.start.max(b.start))..(a.end.min(b.end))
}
pub fn union<T: Ord + Copy>(a: Range<T>, b: Range<T>) -> Range<T> {
    (a.start.min(b.start))..(a.end.max(b.end))
}
pub fn partition<T: Ord + Copy>(range: &Range<T>, middle: T) -> Option<(Range<T>, Range<T>)> {
    if !range.contains(&middle) {
        return None;
    }
    Some(((range.start..middle), (middle..range.end)))
}
