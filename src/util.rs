use std::cmp::Ordering;

pub fn find_min_index<T, F>(array: &[T], compare: F) -> usize
where
    F: Fn(&T, &T) -> Ordering,
{
    assert!(!array.is_empty());

    let mut min_index = 0;
    for i in 1..array.len() {
        match compare(&array[i], &array[min_index]) {
            Ordering::Less => min_index = i,
            _ => {}
        }
    }
    min_index
}
