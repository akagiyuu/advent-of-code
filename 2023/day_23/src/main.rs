use std::collections::HashMap;

use helpers::{Coordinate, Direction};

const INPUT: &str = include_str!("input.txt");

fn flatten(coordinate: Coordinate, grid: &[&[u8]]) -> usize {
    coordinate.0 * grid[0].len() + coordinate.1
}

fn generate_graph(grid: &[&[u8]], is_contain_slope: bool) -> Vec<HashMap<usize, usize>> {
    let mut graph = vec![HashMap::with_capacity(4); grid.len() * grid[0].len()];
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == b'#' {
                continue;
            }
            let current = flatten((i, j), grid);
            let mut directions = Direction::all().to_vec();
            if is_contain_slope && cell != b'.' {
                directions = vec![Direction::from(cell)];
            }
            for direction in directions {
                let Some(next) = direction.apply((i, j), 1, 0..grid.len(), 0..grid[0].len()) else {
                    continue;
                };
                if grid[next.0][next.1] == b'#' {
                    continue;
                }
                let next = flatten(next, grid);
                graph[current].insert(next, 1);
            }
        }
    }
    graph
}

/// Reduce a graph by removing nodes with only 2 neighbors
fn reduce_graph(graph: &mut [HashMap<usize, usize>]) {
    for current_vertex in 0..graph.len() {
        if graph[current_vertex].len() != 2 {
            continue;
        }
        let mut neighbors = graph[current_vertex].iter();
        let (&first_neighbor_vertex, &first_weight) = neighbors.next().unwrap();
        let (&second_neighbor_vertex, &second_weight) = neighbors.next().unwrap();
        let new_connection_length = first_weight + second_weight;

        graph[current_vertex].clear();
        graph[first_neighbor_vertex].remove(&current_vertex);
        graph[second_neighbor_vertex].remove(&current_vertex);
        let value = graph[first_neighbor_vertex]
            .entry(second_neighbor_vertex)
            .or_insert(new_connection_length);
        *value = (*value).max(new_connection_length);
        let value = graph[second_neighbor_vertex]
            .entry(first_neighbor_vertex)
            .or_insert(new_connection_length);
        *value = (*value).max(new_connection_length);
    }
}

fn dfs(
    current: usize,
    is_visited: &mut [bool],
    end: usize,
    graph: &[HashMap<usize, usize>],
) -> Option<usize> {
    if current == end {
        return Some(0);
    }
    let mut path_length = None;
    for (next, weight) in &graph[current] {
        if is_visited[*next] {
            continue;
        }
        is_visited[*next] = true;
        if let Some(length) = dfs(*next, is_visited, end, graph) {
            if path_length.is_none() || length + weight > path_length.unwrap() {
                path_length = Some(length + weight);
            }
        }
        is_visited[*next] = false;
    }
    path_length
}

fn find_longest_path(
    grid: &[&[u8]],
    start: Coordinate,
    end: Coordinate,
    is_contain_slope: bool,
) -> usize {
    let row_count = grid.len();
    let column_count = grid[0].len();
    let mut graph = generate_graph(grid, is_contain_slope);
    reduce_graph(&mut graph);

    dfs(
        flatten(start, grid),
        &mut vec![false; row_count * column_count],
        flatten(end, grid),
        &graph,
    )
    .unwrap()
}

fn main() {
    let grid = INPUT
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();
    let start = grid[0]
        .iter()
        .enumerate()
        .find(|&(_, &x)| x == b'.')
        .unwrap()
        .0;
    let end = grid[grid.len() - 1]
        .iter()
        .enumerate()
        .find(|&(_, &x)| x == b'.')
        .unwrap()
        .0;
    let path_length = find_longest_path(&grid, (0, start), (grid.len() - 1, end), true);
    println!("{}", path_length);

    let path_length = find_longest_path(&grid, (0, start), (grid.len() - 1, end), false);
    println!("{}", path_length);
}
