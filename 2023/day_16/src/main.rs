use std::ops::Range;

const INPUT: &str = include_str!("input.txt");

type Coordinate = (usize, usize);
#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn apply(
        &self,
        coordinate: Coordinate,
        row_boundary: Range<usize>,
        column_boundary: Range<usize>,
    ) -> Option<Coordinate> {
        match self {
            Direction::Up if coordinate.0 > row_boundary.start => {
                Some((coordinate.0 - 1, coordinate.1))
            }
            Direction::Down if coordinate.0 + 1 < row_boundary.end => {
                Some((coordinate.0 + 1, coordinate.1))
            }
            Direction::Left if coordinate.1 > column_boundary.start => {
                Some((coordinate.0, coordinate.1 - 1))
            }
            Direction::Right if coordinate.1 + 1 < column_boundary.end => {
                Some((coordinate.0, coordinate.1 + 1))
            }
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Copy)]
struct Light {
    location: Coordinate,
    pre_direction: Direction,
}

fn count_energized_tile(tiles: &[Vec<u8>], starting_light: Light) -> usize {
    let row_count = tiles.len();
    let column_count = tiles[0].len();
    let mut lights = vec![starting_light];
    let mut is_energized = vec![vec![[false; 4]; tiles[0].len()]; tiles.len()];
    is_energized[starting_light.location.0][starting_light.location.1]
        [starting_light.pre_direction as usize] = true;
    while let Some(mut light) = lights.pop() {
        loop {
            match tiles[light.location.0][light.location.1] {
                b'\\' => {
                    light.pre_direction = match light.pre_direction {
                        Direction::Up => Direction::Left,
                        Direction::Down => Direction::Right,
                        Direction::Left => Direction::Up,
                        Direction::Right => Direction::Down,
                    }
                }
                b'/' => {
                    light.pre_direction = match light.pre_direction {
                        Direction::Up => Direction::Right,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Down,
                        Direction::Right => Direction::Up,
                    }
                }
                b'|' => {
                    match light.pre_direction {
                        Direction::Left | Direction::Right => {
                            let alternative_direction = Direction::Up;
                            if let Some(location) = alternative_direction.apply(
                                light.location,
                                0..row_count,
                                0..column_count,
                            ) {
                                if !is_energized[location.0][location.1]
                                    [alternative_direction as usize]
                                {
                                    is_energized[location.0][location.1]
                                        [alternative_direction as usize] = true;
                                    lights.push(Light {
                                        location,
                                        pre_direction: alternative_direction,
                                    })
                                }
                            }
                            light.pre_direction = Direction::Down;
                        }
                        _ => {}
                    };
                }
                b'-' => match light.pre_direction {
                    Direction::Up | Direction::Down => {
                        let alternative_direction = Direction::Left;
                        if let Some(location) = alternative_direction.apply(
                            light.location,
                            0..row_count,
                            0..column_count,
                        ) {
                            if !is_energized[location.0][location.1][alternative_direction as usize]
                            {
                                is_energized[location.0][location.1]
                                    [alternative_direction as usize] = true;
                                lights.push(Light {
                                    location,
                                    pre_direction: alternative_direction,
                                })
                            }
                        }
                        light.pre_direction = Direction::Right;
                    }
                    _ => {}
                },
                _ => {}
            }
            let Some(new_location) =
                light
                    .pre_direction
                    .apply(light.location, 0..row_count, 0..column_count)
            else {
                break;
            };
            if is_energized[new_location.0][new_location.1][light.pre_direction as usize] {
                break;
            }
            light.location = new_location;

            is_energized[light.location.0][light.location.1][light.pre_direction as usize] = true;
        }
    }
    is_energized
        .iter()
        .flatten()
        .map(|tile| tile.iter().any(|&direction| direction))
        .filter(|&is_energized| is_energized)
        .count()
}

fn find_max_energized_tile_count(tiles: &[Vec<u8>]) -> usize {
    let row_count = tiles.len();
    let column_count = tiles[0].len();
    let mut max_count = 0;
    for row in 0..row_count {
        max_count = max_count.max(count_energized_tile(
            tiles,
            Light {
                location: (row, 0),
                pre_direction: Direction::Right,
            },
        ));

        max_count = max_count.max(count_energized_tile(
            tiles,
            Light {
                location: (row, column_count - 1),
                pre_direction: Direction::Left,
            },
        ));
    }
    for column in 0..column_count {
        max_count = max_count.max(count_energized_tile(
            tiles,
            Light {
                location: (0, column),
                pre_direction: Direction::Down,
            },
        ));

        max_count = max_count.max(count_energized_tile(
            tiles,
            Light {
                location: (row_count - 1, column),
                pre_direction: Direction::Up,
            },
        ));
    }
    max_count
}

fn main() {
    let tiles = INPUT
        .lines()
        .map(|line| line.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let count = count_energized_tile(
        &tiles,
        Light {
            location: (0, 0),
            pre_direction: Direction::Right,
        },
    );
    println!("{}", count);
    println!("{}", find_max_energized_tile_count(&tiles));
}
