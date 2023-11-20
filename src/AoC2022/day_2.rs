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

fn calculate_score_round1(round: Round) -> usize {
    determine_result(round) * 3 + round.1 + 1
}

pub fn calculate_score1(input: &str) -> usize {
    input
        .lines()
        .map(parse_round)
        .map(calculate_score_round1)
        .sum()
}

fn calculate_score_round2(round: Round) -> usize {
    round.1 * 3
        + 1
        + match round.1 {
            0 => (round.0 + 2) % 3,
            1 => round.0,
            2 => (round.0 + 1) % 3,
            _ => 0,
        }
}

pub fn calculate_score2(input: &str) -> usize {
    input
        .lines()
        .map(parse_round)
        .map(calculate_score_round2)
        .sum()
}
