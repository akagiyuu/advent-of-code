const INPUT: &str = include_str!("../input.txt");

fn parse_combo_operand(operand: u64, registers: &[u64; 3]) -> u64 {
    match operand {
        0..=3 => operand,
        operand => registers[operand as usize - 4],
    }
}

fn process_instruction(
    opcode: u64,
    operand: u64,
    mut registers: [u64; 3],
    mut output: Vec<u64>,
    mut pc: usize,
) -> ([u64; 3], Vec<u64>, usize) {
    pc += 1;

    match opcode {
        0 => registers[0] /= 1 << parse_combo_operand(operand, &registers),
        1 => registers[1] ^= operand,
        2 => registers[1] = parse_combo_operand(operand, &registers) % 8,
        3 if registers[0] > 0 => pc = operand as usize,
        4 => registers[1] ^= registers[2],
        5 => output.push(parse_combo_operand(operand, &registers) % 8),
        6 => registers[1] = registers[0] / (1 << parse_combo_operand(operand, &registers)),
        7 => registers[2] = registers[0] / (1 << parse_combo_operand(operand, &registers)),
        _ => {}
    }

    (registers, output, pc)
}

fn get_machine_output(mut registers: [u64; 3], instructions: &[(u64, u64)]) -> Vec<u64> {
    let mut output = Vec::new();
    let mut pc = 0;

    while pc < instructions.len() {
        let (opcode, operand) = instructions[pc];
        (registers, output, pc) = process_instruction(opcode, operand, registers, output, pc);
    }

    output
}

fn is_possible_to_recover(
    mut registers: [u64; 3],
    instructions: &[(u64, u64)],
    instructions_raw: &[u64],
) -> bool {
    let mut output = Vec::new();
    let mut pc = 0;

    while pc < instructions.len() {
        let (opcode, operand) = instructions[pc];
        (registers, output, pc) = process_instruction(opcode, operand, registers, output, pc);

        let out_len = output.len();
        if out_len > instructions_raw.len()
            || (out_len > 0 && output[out_len - 1] != instructions_raw[out_len - 1])
        {
            return false;
        }
    }

    let out_len = output.len();
    if out_len != instructions_raw.len() || output[out_len - 1] != instructions_raw[out_len - 1] {
        return false;
    }

    true
}

fn find_register_a(
    mut registers: [u64; 3],
    instructions: &[(u64, u64)],
    instructions_raw: &[u64],
) -> u64 {
    loop {
        if is_possible_to_recover(registers, instructions, instructions_raw) {
            return registers[0];
        }

        registers[0] += 1;
    }
}

fn main() {
    let (registers, instructions) = INPUT.split_once("\n\n").unwrap();

    let mut registers: [u64; 3] = registers
        .trim()
        .lines()
        .map(|register| register.split_once(": ").unwrap().1.parse::<u64>().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let instructions_raw: Vec<_> = instructions
        .trim()
        .split_once(": ")
        .unwrap()
        .1
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    let instructions: Vec<_> = instructions_raw
        .chunks_exact(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .collect();

    println!(
        "{}",
        get_machine_output(registers, &instructions)
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );

    registers[0] = 1;
    println!(
        "{}",
        find_register_a(registers, &instructions, &instructions_raw)
    );
}
