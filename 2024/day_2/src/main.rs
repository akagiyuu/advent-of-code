const INPUT: &str = include_str!("../input.txt");

fn is_report_safe(report: &[usize]) -> bool {
    if !(1..=3).contains(&report[1].abs_diff(report[0])) {
        return false;
    }

    for i in 2..report.len() {
        if !(1..=3).contains(&report[i].abs_diff(report[i - 1])) {
            return false;
        }
        if report[i].cmp(&report[i - 1]) != report[i - 1].cmp(&report[i - 2]) {
            return false;
        }
    }

    true
}

fn is_report_safe_with_removal(report: &[usize]) -> bool {
    for i in 0..=report.len() {
        let temp = Vec::from_iter(
            report
                .iter()
                .take(i)
                .chain(report.iter().skip(i + 1))
                .copied(),
        );
        if is_report_safe(&temp) {
            return true;
        }
    }

    false
}

fn main() {
    // let safe_count = INPUT
    //     .lines()
    //     .map(|line| {
    //         line.split_whitespace()
    //             .map(|x| x.parse::<usize>().unwrap())
    //             .collect::<Vec<_>>()
    //     })
    //     .filter(|report| is_report_safe(report))
    //     .count();
    // println!("{}", safe_count);

    let safe_count = INPUT
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|report| is_report_safe_with_removal(report))
        .count();
    println!("{}", safe_count);
}
