use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

fn summarize(note: &[&[u8]]) -> usize {
    let row_count = note.len();
    let column_count = note[0].len();

    let mut possible_horizontal_mirror_pair_count = HashMap::new();
    for i in 0..row_count {
        let mut j = i + 1;
        while j < row_count {
            if note[i] == note[j] {
                let mirror_upper = (i + j) / 2;
                *possible_horizontal_mirror_pair_count
                    .entry(mirror_upper)
                    .or_insert(0) += 1;
            }
            j += 2;
        }
    }
    for (mirror_upper, count) in possible_horizontal_mirror_pair_count {
        if count == (mirror_upper + 1).min(row_count - mirror_upper - 1) {
            return (mirror_upper + 1) * 100;
        }
    }

    let mut possible_vertical_mirror_pair_count = HashMap::new();
    for i in 0..column_count {
        let mut j = i + 1;
        while j < column_count {
            let mut is_equal = true;
            for k in 0..row_count {
                is_equal &= note[k][i] == note[k][j];
            }
            if is_equal {
                let mirror_left = (i + j) / 2;
                *possible_vertical_mirror_pair_count
                    .entry(mirror_left)
                    .or_insert(0) += 1;
            }
            j += 2;
        }
    }
    for (mirror_left, count) in possible_vertical_mirror_pair_count {
        if count == (mirror_left + 1).min(column_count - mirror_left - 1) {
            return mirror_left + 1;
        }
    }
    panic!()
}

fn fix_smugde_and_summarize(note: &[&[u8]]) -> usize {
    let row_count = note.len();
    let column_count = note[0].len();

    let mut possible_horizontal_mirror_pair_count = HashMap::new();
    for i in 0..row_count {
        let mut j = i + 1;
        while j < row_count {
            let mut diff = 0;
            for k in 0..column_count {
                if note[i][k] != note[j][k] {
                    diff += 1;
                }
            }
            match diff {
                0 => {
                    let mirror_upper = (i + j) / 2;
                    possible_horizontal_mirror_pair_count
                        .entry(mirror_upper)
                        .or_insert((0, false))
                        .0 += 1;
                }
                1 => {
                    let mirror_upper = (i + j) / 2;
                    possible_horizontal_mirror_pair_count
                        .entry(mirror_upper)
                        .or_insert((0, false))
                        .1 = true;
                }
                _ => {}
            }
            j += 2;
        }
    }
    for (mirror_upper, (count, is_contain_fixable_pair)) in possible_horizontal_mirror_pair_count {
        if count + 1 == (mirror_upper + 1).min(row_count - mirror_upper - 1)
            && is_contain_fixable_pair
        {
            return (mirror_upper + 1) * 100;
        }
    }

    let mut possible_vertical_mirror_pair_count = HashMap::new();
    for i in 0..column_count {
        let mut j = i + 1;
        while j < column_count {
            let mut diff = 0;
            for k in 0..row_count {
                if note[k][i] != note[k][j] {
                    diff += 1;
                }
            }
            match diff {
                0 => {
                    let mirror_left = (i + j) / 2;
                    possible_vertical_mirror_pair_count
                        .entry(mirror_left)
                        .or_insert((0, false))
                        .0 += 1;
                }
                1 => {
                    let mirror_left = (i + j) / 2;
                    possible_vertical_mirror_pair_count
                        .entry(mirror_left)
                        .or_insert((0, false))
                        .1 = true;
                }
                _ => {}
            }
            j += 2;
        }
    }
    for (mirror_left, (count, is_contain_fixable_pair)) in possible_vertical_mirror_pair_count {
        if count + 1 == (mirror_left + 1).min(column_count - mirror_left - 1)
            && is_contain_fixable_pair
        {
            return mirror_left + 1;
        }
    }
    panic!()
}

fn main() {
    let notes = INPUT
        .trim()
        .split("\n\n")
        .map(|note| {
            note.split('\n')
                .map(|line| line.as_bytes())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    println!(
        "{}",
        notes.iter().map(|note| summarize(note)).sum::<usize>()
    );
    println!(
        "{}",
        notes
            .iter()
            .map(|note| fix_smugde_and_summarize(note))
            .sum::<usize>()
    )
}
