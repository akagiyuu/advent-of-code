use crate::Coordinate;
use std::ops::Range;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ReducedDirection {
    Vertical,
    Horizontal,
}
impl ReducedDirection {
    pub fn invert(&self) -> ReducedDirection {
        match self {
            ReducedDirection::Vertical => ReducedDirection::Horizontal,
            ReducedDirection::Horizontal => ReducedDirection::Vertical,
        }
    }
    pub fn apply(
        &self,
        coordinate: Coordinate,
        count: i64,
        row_boundary: Range<usize>,
        column_boundary: Range<usize>,
    ) -> Option<Coordinate> {
        match self {
            ReducedDirection::Vertical => {
                let new_row = coordinate.0 as i64 + count;
                if new_row < 0 {
                    return None;
                }
                let new_row = new_row as usize;
                if row_boundary.contains(&new_row) {
                    return Some((new_row, coordinate.1));
                }
                None
            }
            ReducedDirection::Horizontal => {
                let new_column = coordinate.1 as i64 + count;
                if new_column < 0 {
                    return None;
                }
                let new_column = new_column as usize;
                if column_boundary.contains(&new_column) {
                    return Some((coordinate.0, new_column));
                }
                None
            }
        }
    }
}
