use std::collections::HashMap;

pub fn parse_input(input: &str) -> (HashMap<usize, usize>, HashMap<usize, usize>) {
    let mut freq_first = HashMap::new();
    let mut freq_second = HashMap::new();

    for line in input.lines() {
        let (first, second) = line.split_once("   ").unwrap();
        let first = first.parse::<usize>().unwrap();
        let second = second.parse::<usize>().unwrap();
        *freq_first.entry(first).or_insert(0) += 1;
        *freq_second.entry(second).or_insert(0) += 1;
    }

    (freq_first, freq_second)
}

pub fn sum_distance(mut first: Vec<usize>, mut second: Vec<usize>) -> usize {
    first.sort();
    second.sort();

    first
        .into_iter()
        .zip(second)
        .map(|(first, second)| first.abs_diff(second))
        .sum()
}

pub fn similarity(freq_first: HashMap<usize, usize>, freq_second: HashMap<usize, usize>) -> usize {
    let mut res = 0;

    for (id, count_first) in freq_first {
        res += id * count_first * freq_second.get(&id).copied().unwrap_or(0);
    }

    res
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::*;

    #[rstest]
    #[case(
        r#"
3   4
4   3
2   5
1   3
3   9
3   3"#,
        11
    )]
    fn part_1(#[case] input: &str, #[case] expected: usize) {
        let (first, second) = parse_input(input);

        assert_eq!(sum_distance(first, second), expected)
    }
}
