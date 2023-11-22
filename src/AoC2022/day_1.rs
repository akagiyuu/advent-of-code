use std::cmp::max;

use crate::util::find_min_index;

pub fn find_max_calories(input: &str) -> usize {
    let input = format!("{}\n", input);
    let mut max_calories = 0;
    let mut current_calories = 0;
    input.lines().for_each(|line| {
        if line.is_empty() {
            max_calories = max(max_calories, current_calories);
            current_calories = 0;
            return;
        }

        current_calories += line.parse::<usize>().unwrap();
    });
    max_calories
}

pub fn find_n_max_calories_sum<const N: usize>(input: &str) -> usize {
    let input = format!("{}\n", input);
    let mut max_n_calories = [0; N];
    let mut current_calories = 0;
    input.lines().for_each(|line| {
        if !line.is_empty() {
            current_calories += line.parse::<usize>().unwrap();
            return;
        }

        let min_index = find_min_index(&max_n_calories, |a, b| a.cmp(&b));
        max_n_calories[min_index] = max(max_n_calories[min_index], current_calories);
        current_calories = 0;
    });
    max_n_calories.iter().sum()
}
