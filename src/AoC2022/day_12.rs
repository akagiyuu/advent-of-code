use std::collections::VecDeque;

fn can_climb(current: char, next: char) -> bool {
    next as u8 <= current as u8 + 1
}

fn generate_distance(height_map: &[Vec<char>], start: (usize, usize)) -> Vec<Vec<usize>> {
    let row_count = height_map.len();
    let column_count = height_map[0].len();

    let mut queue = VecDeque::new();
    let mut is_visited = vec![vec![false; column_count]; row_count];
    let mut distance_from_start = vec![vec![usize::MAX; column_count]; row_count];

    queue.push_back(start);
    is_visited[start.0][start.1] = true;
    distance_from_start[start.0][start.1] = 0;

    while let Some(current) = queue.pop_front() {
        let current_distance = distance_from_start[current.0][current.1];
        let current_height = height_map[current.0][current.1];

        if current.0 + 1 < row_count
            && !is_visited[current.0 + 1][current.1]
            && can_climb(current_height, height_map[current.0 + 1][current.1])
        {
            is_visited[current.0 + 1][current.1] = true;
            distance_from_start[current.0 + 1][current.1] = current_distance + 1;
            queue.push_back((current.0 + 1, current.1));
        }

        if current.0 > 0
            && !is_visited[current.0 - 1][current.1]
            && can_climb(current_height, height_map[current.0 - 1][current.1])
        {
            is_visited[current.0 - 1][current.1] = true;
            distance_from_start[current.0 - 1][current.1] = current_distance + 1;
            queue.push_back((current.0 - 1, current.1));
        }

        if current.1 + 1 < column_count
            && !is_visited[current.0][current.1 + 1]
            && can_climb(current_height, height_map[current.0][current.1 + 1])
        {
            is_visited[current.0][current.1 + 1] = true;
            distance_from_start[current.0][current.1 + 1] = current_distance + 1;
            queue.push_back((current.0, current.1 + 1));
        }

        if current.1 > 0
            && !is_visited[current.0][current.1 - 1]
            && can_climb(current_height, height_map[current.0][current.1 - 1])
        {
            is_visited[current.0][current.1 - 1] = true;
            distance_from_start[current.0][current.1 - 1] = current_distance + 1;
            queue.push_back((current.0, current.1 - 1));
        }
    }
    distance_from_start
}

fn generate_distance_reverse(height_map: &[Vec<char>], end: (usize, usize)) -> Vec<Vec<usize>> {
    let row_count = height_map.len();
    let column_count = height_map[0].len();

    let mut queue = VecDeque::new();
    let mut is_visited = vec![vec![false; column_count]; row_count];
    let mut distance_from_start = vec![vec![usize::MAX; column_count]; row_count];

    queue.push_back(end);
    is_visited[end.0][end.1] = true;
    distance_from_start[end.0][end.1] = 0;

    while let Some(current) = queue.pop_front() {
        let current_distance = distance_from_start[current.0][current.1];
        let current_height = height_map[current.0][current.1];

        if current.0 + 1 < row_count
            && !is_visited[current.0 + 1][current.1]
            && can_climb(height_map[current.0 + 1][current.1], current_height)
        {
            is_visited[current.0 + 1][current.1] = true;
            distance_from_start[current.0 + 1][current.1] = current_distance + 1;
            queue.push_back((current.0 + 1, current.1));
        }

        if current.0 > 0
            && !is_visited[current.0 - 1][current.1]
            && can_climb(height_map[current.0 - 1][current.1], current_height)
        {
            is_visited[current.0 - 1][current.1] = true;
            distance_from_start[current.0 - 1][current.1] = current_distance + 1;
            queue.push_back((current.0 - 1, current.1));
        }

        if current.1 + 1 < column_count
            && !is_visited[current.0][current.1 + 1]
            && can_climb(height_map[current.0][current.1 + 1], current_height)
        {
            is_visited[current.0][current.1 + 1] = true;
            distance_from_start[current.0][current.1 + 1] = current_distance + 1;
            queue.push_back((current.0, current.1 + 1));
        }

        if current.1 > 0
            && !is_visited[current.0][current.1 - 1]
            && can_climb(height_map[current.0][current.1 - 1], current_height)
        {
            is_visited[current.0][current.1 - 1] = true;
            distance_from_start[current.0][current.1 - 1] = current_distance + 1;
            queue.push_back((current.0, current.1 - 1));
        }
    }
    distance_from_start
}

pub fn find_min_step_1(input: &str) -> usize {
    let mut height_map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut start = (0, 0);
    let mut end = (0, 0);

    for (i, row) in height_map.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            match cell {
                'S' => {
                    start = (i, j);
                    *cell = 'a';
                }
                'E' => {
                    end = (i, j);
                    *cell = 'z';
                }
                _ => {}
            }
        }
    }
    let distance = generate_distance(&height_map, start);
    distance[end.0][end.1]
}

pub fn find_min_step_2(input: &str) -> usize {
    let mut height_map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut starts = vec![];
    let mut end = (0, 0);

    for (i, row) in height_map.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            match cell {
                'S' => {
                    starts.push((i, j));
                    *cell = 'a';
                }
                'E' => {
                    end = (i, j);
                    *cell = 'z';
                }
                'a' => {
                    starts.push((i, j));
                }
                _ => {}
            }
        }
    }
    let distance = generate_distance_reverse(&height_map, end);
    starts.iter().map(|&(i, j)| distance[i][j]).min().unwrap()
}
