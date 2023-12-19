use helpers::{intersect, partition};
use std::{collections::HashMap, ops::Range};

const INPUT: &str = include_str!("input.txt");
const FIRST_WORKFLOW_NAME: &str = "in";
const PART_CATEGORY_COUNT: usize = 4;
const PART_VALUE_RANGE: Range<usize> = 1..4001;
fn encode(part_category: u8) -> usize {
    match part_category {
        b'x' => 0,
        b'm' => 1,
        b'a' => 2,
        b's' => 3,
        _ => unreachable!(),
    }
}
type Part = [usize; PART_CATEGORY_COUNT];
#[derive(Debug, Clone)]
struct Condition {
    ranges: [Range<usize>; PART_CATEGORY_COUNT],
    destination: String,
}
impl Condition {
    fn is_satisfied(&self, part: &Part) -> bool {
        for (value, range) in part.iter().zip(self.ranges.iter()) {
            if !range.contains(value) {
                return false;
            }
        }
        true
    }
}

fn parse_workflow_data(input: &str) -> Vec<Condition> {
    let mut workflow_data = vec![];
    let mut remaining_ranges = [PART_VALUE_RANGE; PART_CATEGORY_COUNT];
    for condition_raw in input.split(',') {
        match condition_raw.split_once(':') {
            None => workflow_data.push(Condition {
                ranges: remaining_ranges.clone(),
                destination: condition_raw.to_string(),
            }),
            Some((comparision, destination)) => {
                let category = encode(comparision.as_bytes()[0]);
                let value = comparision[2..].parse().unwrap();

                match comparision.as_bytes()[1] {
                    b'<' => {
                        let (left, right) =
                            partition(&remaining_ranges[category], value).unwrap();
                        if !left.is_empty() {
                            let mut copied_ranges = remaining_ranges.clone();
                            copied_ranges[category] = left;
                            workflow_data.push(Condition {
                                ranges: copied_ranges,
                                destination: destination.to_string(),
                            });
                        }
                        if right.is_empty() {
                            break;
                        }
                        remaining_ranges[category] = right;
                    }
                    b'>' => {
                        let (left, right) =
                            partition(&remaining_ranges[category], value + 1).unwrap();
                        if !right.is_empty() {
                            let mut copied_ranges = remaining_ranges.clone();
                            copied_ranges[category] = right;
                            workflow_data.push(Condition {
                                ranges: copied_ranges,
                                destination: destination.to_string(),
                            });
                        }
                        if left.is_empty() {
                            break;
                        }
                        remaining_ranges[category] = left;
                    }
                    _ => unreachable!(),
                };
            }
        }
    }
    workflow_data
}

fn sum_accepted_parts_rating_number(
    parts: &[Part],
    workflows: &HashMap<String, Vec<Condition>>,
) -> usize {
    let mut accepted_parts = vec![];

    for part in parts {
        let mut current_workflow_name = FIRST_WORKFLOW_NAME;
        loop {
            if current_workflow_name == "R" {
                break;
            }
            if current_workflow_name == "A" {
                accepted_parts.push(part);
                break;
            }
            for condition in &workflows[current_workflow_name] {
                if condition.is_satisfied(part) {
                    current_workflow_name = &condition.destination;
                }
            }
        }
    }
    accepted_parts
        .iter()
        .map(|part| part.iter().sum::<usize>())
        .sum()
}

fn intersect_ranges(
    a: &[Range<usize>; PART_CATEGORY_COUNT],
    b: &[Range<usize>; PART_CATEGORY_COUNT],
) -> [Range<usize>; PART_CATEGORY_COUNT] {
    let mut result = [PART_VALUE_RANGE; PART_CATEGORY_COUNT];
    for i in 0..PART_CATEGORY_COUNT {
        result[i] = intersect(a[i].clone(), b[i].clone());
    }
    result
}

fn count_possible_accepted_combinations(workflows: &HashMap<String, Vec<Condition>>) -> usize {
    let mut count = 0;
    let mut conditions = workflows[FIRST_WORKFLOW_NAME]
        .iter()
        .cloned()
        .collect::<Vec<_>>();
    // Connect conditions of first workflow to end point (R or A)
    while let Some(condition) = conditions.pop() {
        if condition.destination == "R" {
            continue;
        }
        if condition.destination == "A" {
            let mut value = 1;
            for part_range in &condition.ranges {
                value *= part_range.len();
            }
            count += value;
            continue;
        }
        let next_conditions = &workflows[&condition.destination];
        for next_condition in next_conditions {
            conditions.push(Condition {
                ranges: intersect_ranges(&condition.ranges, &next_condition.ranges),
                destination: next_condition.destination.clone(),
            });
        }
    }
    count
}

fn main() {
    let (workflows, parts) = INPUT.split_once("\n\n").unwrap();
    let workflows = workflows
        .lines()
        .map(|workflow| {
            let (name, conditions) = workflow.split_once('{').unwrap();
            let conditions = parse_workflow_data(&conditions[..conditions.len() - 1]);
            (name.to_string(), conditions)
        })
        .collect::<HashMap<_, _>>();
    let parts = parts
        .lines()
        .map(|line| {
            let mut part = [0; 4];
            let mut value = 0;
            let mut index = 0;
            for c in line.bytes() {
                if c.is_ascii_digit() {
                    value = value * 10 + (c - b'0') as usize;
                    continue;
                }
                if value > 0 {
                    part[index] = value;
                    index += 1;
                    value = 0;
                }
            }
            part
        })
        .collect::<Vec<_>>();
    let sum = sum_accepted_parts_rating_number(&parts, &workflows);
    println!("{}", sum);
    let combinations = count_possible_accepted_combinations(&workflows);
    println!("{}", combinations);
}
