const INPUT: &str = include_str!("input.txt");

fn encode(s: u8) -> usize {
    match s {
        b'A' | b'X' => 0,
        b'B' | b'Y' => 1,
        b'C' | b'Z' => 2,
        _ => 0,
    }
}

type Round = (usize, usize);

fn parse_round(s: &str) -> Round {
    (encode(s.as_bytes()[0]), encode(s.as_bytes()[2]))
}

fn determine_result(round: Round) -> usize {
    if round.1 == round.0 {
        return 1;
    }
    if round.1 == (round.0 + 1) % 3 {
        return 2;
    }
    0
}

fn main() {
    let rounds = INPUT.lines().map(parse_round);

    println!(
        "{}",
        rounds
            .clone()
            .map(|round| determine_result(round) * 3 + round.1 + 1)
            .sum::<usize>()
    );
    println!(
        "{}",
        rounds
            .map(|round| round.1 * 3
                + 1
                + match round.1 {
                    0 => (round.0 + 2) % 3,
                    1 => round.0,
                    2 => (round.0 + 1) % 3,
                    _ => 0,
                })
            .sum::<usize>()
    );
}
