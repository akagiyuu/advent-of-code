const INPUT: &str = include_str!("../input.txt");

fn calculate_block_checksum(id: usize, start: usize, length: usize) -> usize {
    id * (start * length + length * (length - 1) / 2)
}

fn calculate_checksum_with_fragmentation(mut memory_map: Vec<usize>) -> usize {
    let n = memory_map.len();

    let mut start_index = vec![0; n];
    for i in 1..n {
        start_index[i] = start_index[i - 1] + memory_map[i - 1];
    }

    let mut checksum = 0;
    let mut i = 1;
    let mut j = (n - 1) / 2 * 2;
    while i <= j {
        let max_replace = memory_map[i].min(memory_map[j]);
        if max_replace > 0 {
            memory_map[i] -= max_replace;
            memory_map[j] -= max_replace;

            checksum += calculate_block_checksum(j / 2, start_index[i], max_replace);
            start_index[i] += max_replace;
        }
        if memory_map[i] == 0 {
            i += 2;
        }
        if memory_map[j] == 0 {
            j -= 2;
        }
    }
    for i in (0..n).step_by(2) {
        if memory_map[i] == 0 {
            continue;
        }
        checksum += calculate_block_checksum(i / 2, start_index[i], memory_map[i]);
    }

    checksum
}

fn calculate_checksum_without_fragmentation(mut memory_map: Vec<usize>) -> usize {
    let n = memory_map.len();

    let mut start_index = vec![0; n];
    for i in 1..n {
        start_index[i] = start_index[i - 1] + memory_map[i - 1];
    }

    let mut checksum = 0;

    for i in (0..n).step_by(2).rev() {
        let id = i / 2;
        let length = memory_map[i];
        let mut start = start_index[i];
        for j in (1..i).step_by(2) {
            if memory_map[j] < length {
                continue;
            }
            start = start_index[j];

            memory_map[j] -= length;
            start_index[j] += length;
            break;
        }

        checksum += calculate_block_checksum(id, start, length);
    }

    checksum
}

fn main() {
    let memory_map: Vec<_> = INPUT.trim().bytes().map(|c| (c - b'0') as usize).collect();

    println!(
        "{}",
        calculate_checksum_with_fragmentation(memory_map.clone())
    );

    println!(
        "{}",
        calculate_checksum_without_fragmentation(memory_map.clone())
    );
}
