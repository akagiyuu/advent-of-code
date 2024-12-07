const INPUT: &str = include_str!("../input.txt");

#[derive(Clone, Copy)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

fn cal(mut a: u64, mut b: u64, operator: Operator) -> u64 {
    match operator {
        Operator::Add => a + b,
        Operator::Multiply => a * b,
        Operator::Concatenate => {
            let mut b_digit = Vec::new();

            while b > 0 {
                b_digit.push(b % 10);
                b /= 10;
            }

            while let Some(digit) = b_digit.pop() {
                a = 10 * a + digit;
            }

            a
        }
    }
}

struct Equation {
    nums: Vec<u64>,
    expected_result: u64,
}

impl From<&str> for Equation {
    fn from(value: &str) -> Self {
        let (expected_result, nums) = value.split_once(": ").unwrap();

        let expected_result = expected_result.parse().unwrap();

        let nums = nums
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        Self {
            nums,
            expected_result,
        }
    }
}

fn is_possible(result: u64, nums: &[u64], expected_result: u64, operators: &[Operator]) -> bool {
    if nums.is_empty() {
        return result == expected_result;
    }

    if result > expected_result {
        return false;
    }

    operators.iter().any(|&operator| {
        is_possible(
            cal(result, nums[0], operator),
            &nums[1..],
            expected_result,
            operators,
        )
    })
}

fn total_calibration_result(equations: &[Equation], operators: &[Operator]) -> u64 {
    equations
        .iter()
        .filter_map(|equation| {
            if is_possible(
                equation.nums[0],
                &equation.nums[1..],
                equation.expected_result,
                operators,
            ) {
                Some(equation.expected_result)
            } else {
                None
            }
        })
        .sum()
}

fn main() {
    let equations: Vec<_> = INPUT.lines().map(Equation::from).collect();

    println!(
        "{}",
        total_calibration_result(&equations, &[Operator::Add, Operator::Multiply])
    );
    println!(
        "{}",
        total_calibration_result(
            &equations,
            &[Operator::Add, Operator::Multiply, Operator::Concatenate]
        )
    );
}
