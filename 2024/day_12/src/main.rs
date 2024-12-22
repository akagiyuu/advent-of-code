use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");

const MAIN_DIRECTION: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn into_valid((x, y): (isize, isize), group_c: u8, garden: &[Vec<u8>]) -> Option<(usize, usize)> {
    let n = garden.len() as isize;
    let m = garden[0].len() as isize;

    if !(0..n).contains(&x) || !(0..m).contains(&y) {
        return None;
    }

    if garden[x as usize][y as usize] == group_c {
        Some((x as usize, y as usize))
    } else {
        None
    }
}

fn perimeter((x, y): (usize, usize), group_c: u8, garden: &[Vec<u8>]) -> usize {
    MAIN_DIRECTION
        .into_iter()
        .filter(|(dx, dy)| {
            into_valid((x as isize + dx, y as isize + dy), group_c, garden).is_none()
        })
        .count()
}

fn corner((x, y): (usize, usize), group_c: u8, garden: &[Vec<u8>]) -> usize {
    let [nw, w, sw, n, s, ne, e, se]: [bool; 8] = (-1..2)
        .cartesian_product(-1..2)
        .filter(|(dx, dy)| dx | dy != 0)
        .map(|(dx, dy)| into_valid((x as isize + dx, y as isize + dy), group_c, garden).is_some())
        .collect_vec()
        .try_into()
        .unwrap();
    [
        n && w && !nw,
        n && e && !ne,
        s && w && !sw,
        s && e && !se,
        !(n || w),
        !(n || e),
        !(s || w),
        !(s || e),
    ]
    .into_iter()
    .filter(|&x| x)
    .count()
}

fn calculate_total_price(
    garden: &[Vec<u8>],
    cell_value: impl Fn((usize, usize), u8, &[Vec<u8>]) -> usize,
) -> usize {
    let n = garden.len();
    let m = garden[0].len();

    let mut price = 0;

    let mut is_visited = vec![vec![false; m]; n];
    let mut stack = vec![];

    for (cx, row) in garden.iter().enumerate() {
        for (cy, &group_c) in row.iter().enumerate() {
            if is_visited[cx][cy] {
                continue;
            }

            let mut group_area = 0;
            let mut group_value = 0;
            stack.clear();
            stack.push((cx, cy));

            while let Some((x, y)) = stack.pop() {
                if is_visited[x][y] {
                    continue;
                }
                is_visited[x][y] = true;

                group_area += 1;
                group_value += cell_value((x, y), group_c, garden);

                let neighbors = MAIN_DIRECTION.into_iter().filter_map(|(dx, dy)| {
                    into_valid((x as isize + dx, y as isize + dy), group_c, garden)
                });
                stack.extend(neighbors);
            }
            price += group_area * group_value;
        }
    }

    price
}

fn main() {
    let garden: Vec<_> = INPUT.lines().map(|line| line.as_bytes().to_vec()).collect();

    println!("{}", calculate_total_price(&garden, perimeter));
    println!("{}", calculate_total_price(&garden, corner));
}
