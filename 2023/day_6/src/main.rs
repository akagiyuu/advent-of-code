const INPUT: &str = include_str!("input.txt");

fn parse_multi_space_separated_list(raw: &str) -> Vec<usize> {
    let mut result = vec![];
    let mut current_value = 0;
    for c in raw.bytes() {
        if c.is_ascii_digit() {
            current_value = current_value * 10 + (c - b'0') as usize;
            continue;
        }
        if current_value > 0 {
            result.push(current_value);
            current_value = 0;
        }
    }
    if current_value > 0 {
        result.push(current_value);
    }
    result
}

fn count_way_to_beat_record(total_time: usize, record: usize) -> usize {
    let delta = ((total_time * total_time - 4 * record) as f64).sqrt();
    let left = ((total_time as f64 - delta) / 2.).floor() as usize + 1;
    let right = ((total_time as f64 + delta) / 2.).ceil() as usize - 1;
    right - left + 1
}

fn main() {
    let (times_raw, records_raw) = INPUT.split_once('\n').unwrap();
    let times = parse_multi_space_separated_list(times_raw);
    let records = parse_multi_space_separated_list(records_raw);
    let product = times
        .iter()
        .zip(records.iter())
        .fold(1, |acc, (&time, &record)| {
            acc * count_way_to_beat_record(time, record)
        });
    println!("{}", product);

    let time = times_raw
        .bytes()
        .filter(|c| c.is_ascii_digit())
        .fold(0, |acc, c| acc * 10 + (c - b'0') as usize);
    let record = records_raw
        .bytes()
        .filter(|c| c.is_ascii_digit())
        .fold(0, |acc, c| acc * 10 + (c - b'0') as usize);
    println!("{}", count_way_to_beat_record(time, record));
}
