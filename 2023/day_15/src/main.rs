const INPUT: &str = include_str!("input.txt");
const MODULUS: usize = 256;
const MULTIPLIER: usize = 17;

fn hash(sequence: &[u8]) -> usize {
    sequence.iter().fold(0, |hashed, &c| {
        ((hashed + c as usize) % MODULUS) * MULTIPLIER % MODULUS
    })
}

#[derive(Clone, Debug)]
struct Lens<'a> {
    label: &'a [u8],
    focal_length: u8,
}

fn calculate_focusing_power(sequences: &str) -> usize {
    let mut boxes: Vec<Vec<Lens>> = vec![vec![]; MODULUS];
    'outer: for sequence in sequences.split(',') {
        let mut label_end = 0;
        let sequence = sequence.as_bytes();
        while sequence[label_end] != b'=' && sequence[label_end] != b'-' {
            label_end += 1;
        }

        let label = &sequence[..label_end];
        let hashed_label = hash(label);
        let operation = sequence[label_end];
        match operation {
            b'=' => {
                let focal_length = sequence[label_end + 1] - b'0';
                for element in boxes[hashed_label].iter_mut() {
                    if element.label == label {
                        element.focal_length = focal_length;
                        continue 'outer;
                    }
                }
                boxes[hashed_label].push(Lens {
                    label,
                    focal_length,
                });
            }
            b'-' => {
                for i in 0..boxes[hashed_label].len() {
                    if boxes[hashed_label][i].label == label {
                        boxes[hashed_label].remove(i);
                        break;
                    }
                }
            }
            _ => unreachable!(),
        }
    }
    boxes
        .iter()
        .enumerate()
        .filter(|(_, b)| !b.is_empty())
        .map(|(i, b)| {
            (i + 1)
                * b.iter()
                    .enumerate()
                    .map(|(j, lens)| (j + 1) * lens.focal_length as usize)
                    .sum::<usize>()
        })
        .sum::<usize>()
}

fn main() {
    let sum = INPUT
        .trim()
        .split(',')
        .map(|sequence| sequence.as_bytes())
        .map(hash)
        .sum::<usize>();
    println!("{}", sum);
    println!("{}", calculate_focusing_power(INPUT.trim()));
}
