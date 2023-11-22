pub fn find_min_index(array: &[usize]) -> usize {
    assert!(!array.is_empty());

    let mut min_index = 0;
    for i in 1..array.len() {
        if array[i] < array[min_index] {
            min_index = i;
        }
    }
    min_index
}
pub fn find_n_max<const N: usize>(array: &[usize]) -> [usize; N] {
    assert!(!array.is_empty());
    assert!(N > 0);
    assert!(N <= array.len());

    let mut result: [usize; N] = [0; N];
    for i in 0..N {
        result[i] = array[i];
    }

    for i in N..array.len() {
        let min_index = find_min_index(&result);
        if array[i] > result[min_index] {
            result[min_index] = array[i];
        }
    }

    result
}
