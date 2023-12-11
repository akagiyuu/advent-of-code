const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Data {
    kind: char,
    x: usize,
    y: usize,
}
impl Data {
    fn distance(&self, other: &Data) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

fn parse_image(raw: &str, expand_amount: usize) -> Vec<Vec<Data>> {
    let mut image = INPUT
        .lines()
        .enumerate()
        .map(|(x, line)| {
            line.chars()
                .enumerate()
                .map(|(y, c)| Data { kind: c, x, y })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let row_count = image.len();
    let column_count = image[0].len();
    'outer: for i in 0..row_count {
        for j in 0..column_count {
            if image[i][j].kind == '#' {
                continue 'outer;
            }
        }
        for r in i + 1..row_count {
            for c in 0..column_count {
                image[r][c].x += expand_amount;
            }
        }
    }
    'outer: for j in 0..column_count {
        for i in 0..row_count {
            if image[i][j].kind == '#' {
                continue 'outer;
            }
        }
        for c in j + 1..column_count {
            for r in 0..row_count {
                image[r][c].y += expand_amount;
            }
        }
    }
    image
}

fn calculate_total_distance(image: &[Vec<Data>]) -> usize {
    let galaxies = image
        .iter()
        .flatten()
        .filter(|x| x.kind == '#')
        .collect::<Vec<_>>();
    let mut total_distance = 0;
    let galaxy_count = galaxies.len();
    for i in 0..galaxy_count {
        for j in i + 1..galaxy_count {
            total_distance += galaxies[i].distance(galaxies[j]);
        }
    }
    total_distance
}

fn main() {
    println!("{}", calculate_total_distance(&parse_image(INPUT, 2 - 1)));
    println!(
        "{}",
        calculate_total_distance(&parse_image(INPUT, 1000000 - 1))
    );
}
