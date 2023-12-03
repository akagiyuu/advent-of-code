const INPUT: &str = include_str!("input.txt");

fn minimum_cube_needed(game: &str) -> usize {
    let (_, cube_sets) = game.split_once(": ").unwrap();
    let mut min_cube = [0; 3];
    for set in cube_sets.split("; ") {
        for cube in set.split(", ") {
            let (count, color) = cube.split_once(' ').unwrap();
            let count = count.parse::<usize>().unwrap();
            match color {
                "red" => min_cube[0] = min_cube[0].max(count),
                "green" => min_cube[1] = min_cube[1].max(count),
                "blue" => min_cube[2] = min_cube[2].max(count),
                _ => panic!(),
            }
        }
    }
    min_cube[0] * min_cube[1] * min_cube[2]
}

fn is_game_possible(game: &str) -> bool {
    let (_, cube_sets) = game.split_once(": ").unwrap();
    cube_sets
        .split("; ")
        .map(|set| {
            set.split(", ")
                .map(|cube| {
                    let (count, color) = cube.split_once(' ').unwrap();
                    let count = count.parse::<usize>().unwrap();
                    match color {
                        "red" => count <= 12,
                        "green" => count <= 13,
                        "blue" => count <= 14,
                        _ => panic!(),
                    }
                })
                .fold(true, |acc, is_possible| is_possible && acc)
        })
        .fold(true, |acc, is_set_possible| is_set_possible && acc)
}

fn main() {
    println!(
        "{}",
        INPUT
            .trim()
            .split('\n')
            .enumerate()
            .filter(|(_, game)| is_game_possible(game))
            .fold(0, |sum, (id, _)| sum + id + 1)
    );

    println!(
        "{}",
        INPUT
            .trim()
            .split('\n')
            .map(minimum_cube_needed)
            .sum::<usize>()
    );
}
