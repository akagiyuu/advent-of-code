use std::cmp::Ordering;

pub fn find_min_index<F>(array: &[usize], compare: F) -> usize
where
    F: Fn(usize, usize) -> Ordering,
{
    assert!(!array.is_empty());

    let mut min_index = 0;
    for i in 1..array.len() {
        match compare(array[i], array[min_index]) {
            Ordering::Less => min_index = i,
            _ => {}
        }
    }
    min_index
}
pub fn find_n_max<const N: usize, F>(array: &[usize], compare: F) -> [usize; N]
where
    F: Fn(usize, usize) -> Ordering,
{
    assert!(!array.is_empty());
    assert!(N > 0);
    assert!(N <= array.len());

    let mut result: [usize; N] = [0; N];
    for i in 0..N {
        result[i] = array[i];
    }

    for i in N..array.len() {
        let min_index = find_min_index(&result, compare);
        match compare(array[i], result[min_index]) {
            Ordering::Greater => result[min_index] = array[i],
            _ => {}
        }
    }

    result
}
