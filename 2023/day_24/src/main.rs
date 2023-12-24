use std::{
    ops::{Deref, RangeInclusive},
    process::Command,
};

const INPUT: &str = include_str!("input.txt");

fn solve_linear_system(a: [f64; 2], b: [f64; 2], c: [f64; 2]) -> Option<[f64; 2]> {
    let determinant = a[0] * b[1] - b[0] * a[1];
    if determinant == 0.0 {
        return None;
    }
    let x = (c[0] * b[1] - b[0] * c[1]) / determinant;
    let y = (a[0] * c[1] - c[0] * a[1]) / determinant;
    Some([x, y])
}

#[derive(Debug)]
struct Line {
    start: (f64, f64, f64),
    direction_vector: (f64, f64, f64),
}
impl From<&str> for Line {
    fn from(value: &str) -> Self {
        let (start, direction_vector) = value.split_once(" @ ").unwrap();

        let [x, y, z] = *start
            .split(", ")
            .take(3)
            .map(|x| x.trim().parse::<f64>().unwrap())
            .collect::<Vec<_>>()
            .deref()
        else {
            unreachable!()
        };
        let [vx, vy, vz] = *direction_vector
            .split(", ")
            .take(3)
            .map(|x| x.trim().parse::<f64>().unwrap())
            .collect::<Vec<_>>()
            .deref()
        else {
            unreachable!()
        };

        Line {
            start: (x, y, z),
            direction_vector: (vx, vy, vz),
        }
    }
}
impl Line {
    fn projected_positive_intersection(&self, other: &Self) -> Option<(f64, f64)> {
        let Some([t1, t2]) = solve_linear_system(
            [self.direction_vector.0, self.direction_vector.1],
            [-other.direction_vector.0, -other.direction_vector.1],
            [other.start.0 - self.start.0, other.start.1 - self.start.1],
        ) else {
            return None;
        };
        if t1 < 0.0 || t2 < 0.0 {
            return None;
        }
        Some((
            self.start.0 + t1 * self.direction_vector.0,
            self.start.1 + t1 * self.direction_vector.1,
        ))
    }
}

/// Solve system of equations with 9 equations and 9 variables using wolframscript
fn find_line_intersect_all_lines(lines: &[Line]) -> Line {
    assert!(lines.len() >= 3);
    let output = Command::new("wolframscript")
        .arg("-c")
        .arg(format!(
            "Solve[
        x + vx * t1 == {} + {} * t1 &&
        y + vy * t1 == {} + {} * t1 &&
        z + vz * t1 == {} + {} * t1 &&

        x + vx * t2 == {} + {} * t2 &&
        y + vy * t2 == {} + {} * t2 &&
        z + vz * t2 == {} + {} * t2 &&

        x + vx * t3 == {} + {} * t3 &&
        y + vy * t3 == {} + {} * t3 &&
        z + vz * t3 == {} + {} * t3
        ,
        {{ x, y, z, vx, vy, vz, t1, t2, t3 }}
]",
            lines[0].start.0,
            lines[0].direction_vector.0,
            lines[0].start.1,
            lines[0].direction_vector.1,
            lines[0].start.2,
            lines[0].direction_vector.2,
            lines[1].start.0,
            lines[1].direction_vector.0,
            lines[1].start.1,
            lines[1].direction_vector.1,
            lines[1].start.2,
            lines[1].direction_vector.2,
            lines[2].start.0,
            lines[2].direction_vector.0,
            lines[2].start.1,
            lines[2].direction_vector.1,
            lines[2].start.2,
            lines[2].direction_vector.2
        ))
        .output()
        .unwrap();
    // Output is in form {{x -> 24, y -> 13, z -> 10, vx -> -3, vy -> 1, vz -> 2, t1 -> 5, t2 -> 3, t3 -> 4}}
    let output = String::from_utf8(output.stdout).unwrap();
    let output = output.trim();
    let output = &output[2..output.len() - 2];
    let [x, y, z, vx, vy, vz] = *output
        .split(", ")
        .take(6)
        .map(|variable| {
            variable
                .split_once(" -> ")
                .unwrap()
                .1
                .parse::<f64>()
                .unwrap()
        })
        .collect::<Vec<_>>()
        .deref()
    else {
        unreachable!();
    };
    Line {
        start: (x, y, z),
        direction_vector: (vx, vy, vz),
    }
}

fn count_projected_positive_intersection_in_area(
    lines: &[Line],
    x_range: RangeInclusive<f64>,
    y_range: RangeInclusive<f64>,
) -> i64 {
    let mut count = 0;
    let line_count = lines.len();
    for i in 0..line_count {
        for j in i + 1..line_count {
            let Some((x, y)) = lines[i].projected_positive_intersection(&lines[j]) else {
                continue;
            };
            if x_range.contains(&x) && y_range.contains(&y) {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let lines = INPUT.lines().map(Line::from).collect::<Vec<_>>();
    let count = count_projected_positive_intersection_in_area(
        &lines,
        200000000000000f64..=400000000000000f64,
        200000000000000f64..=400000000000000f64,
    );
    println!("{}", count);
    let line = find_line_intersect_all_lines(&lines);
    println!("{:?}", line.start.0 + line.start.1 + line.start.2);
}
