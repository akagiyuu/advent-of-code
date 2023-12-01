const INPUT: &str = include_str!("input.txt");

const NUMBERS: [&[u8]; 10] = [
    b" ", b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];
fn get_calibration_value(line: &[u8]) -> usize {
    let mut first = None;
    let mut second = None;
    for c in line {
        if !c.is_ascii_digit() {
            continue;
        }
        if first.is_none() {
            first = Some(c - b'0');
        }
        second = Some(c - b'0');
    }
    (first.unwrap() * 10 + second.unwrap()) as usize
}

fn get_calibration_value_with_word(line: &[u8]) -> usize {
    let mut first = None;
    let mut second = None;

    for i in 0..line.len() {
        let char = line[i];
        if char.is_ascii_digit() {
            let value = Some((char - b'0') as usize);
            if first.is_none() {
                first = value;
            }
            second = value;
            continue;
        }
        for (number, &word) in NUMBERS.iter().enumerate() {
            if !char == word[0] {
                continue;
            }
            let word_len = word.len();
            if i + word_len > line.len() {
                continue;
            }
            if &line[i..i + word_len] != word {
                continue;
            }
            if first.is_none() {
                first = Some(number);
            }
            second = Some(number);
        }
    }
    first.unwrap() * 10 + second.unwrap()
}

fn main() {
    let calibration_sum = INPUT
        .lines()
        .map(|line| get_calibration_value(line.as_bytes()))
        .sum::<usize>();
    println!("{}", calibration_sum);

    let calibration_sum = INPUT
        .lines()
        .map(|line| get_calibration_value_with_word(line.as_bytes()))
        .sum::<usize>();
    println!("{}", calibration_sum);
}
