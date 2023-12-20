mod direction;

pub use direction::*;
use std::ops::Range;
pub type Coordinate = (usize, usize);
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
