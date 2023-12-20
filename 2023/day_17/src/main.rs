use helpers::ReducedDirection;
use std::collections::{BinaryHeap, HashMap, HashSet};

const INPUT: &str = include_str!("input.txt");

type Coordinate = (usize, usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Node {
    coordinate: Coordinate,
    direction: ReducedDirection,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct QueueElement {
    node: Node,
    heat_loss: usize,
}
impl PartialOrd for QueueElement {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.heat_loss.cmp(&self.heat_loss))
    }
}
impl Ord for QueueElement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.heat_loss.cmp(&self.heat_loss)
    }
}

fn find_min_heat_loss(
    heat_loss_map: &[Vec<usize>],
    min_move: usize,
    max_move: usize,
    start: Coordinate,
    end: Coordinate,
) -> usize {
    let row_count = heat_loss_map.len();
    let column_count = heat_loss_map[0].len();

    let start_right = Node {
        coordinate: start,
        direction: ReducedDirection::Vertical,
    };
    let start_down = Node {
        coordinate: start,
        direction: ReducedDirection::Horizontal,
    };

    let mut min = HashMap::from([(start_right, 0), (start_down, 0)]);
    let mut visited = HashSet::new();
    let mut queue = BinaryHeap::from([
        QueueElement {
            node: start_right,
            heat_loss: 0,
        },
        QueueElement {
            node: start_down,
            heat_loss: 0,
        },
    ]);

    while let Some(QueueElement {
        node: current,
        // heat_loss is the minimum heat loss for current node with some streak count
        heat_loss,
    }) = queue.pop()
    {
        if current.coordinate == end {
            return min[&current];
        }

        if visited.contains(&current) {
            continue;
        }
        visited.insert(current);

        let direction = current.direction.invert();
        for i in [-1, 1] {
            let mut current_heat_loss = heat_loss;
            for j in 1..=(max_move as i64) {
                let coordinate =
                    match direction.apply(current.coordinate, i * j, 0..row_count, 0..column_count)
                    {
                        Some(coordinate) => coordinate,
                        None => break,
                    };
                let neighbor = Node {
                    coordinate,
                    direction,
                };
                current_heat_loss += heat_loss_map[neighbor.coordinate.0][neighbor.coordinate.1];
                if j < (min_move as i64) {
                    continue;
                }

                if !min.contains_key(&neighbor) || min[&neighbor] > current_heat_loss {
                    min.insert(neighbor, current_heat_loss);
                    queue.push(QueueElement {
                        node: neighbor,
                        heat_loss: current_heat_loss,
                    });
                }
            }
        }
    }
    usize::MAX
    // min.iter()
    //     .filter(|element| element.0.coordinate == end)
    //     .fold(usize::MAX, |m, element| {
    //         println!("{:?}", element);
    //         m.min(*element.1)
    //     })
}

fn main() {
    let heat_loss_map = INPUT
        .lines()
        .map(|line| {
            line.bytes()
                .map(|b| (b - b'0') as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let heat_loss = find_min_heat_loss(
        &heat_loss_map,
        1,
        3,
        (0, 0),
        (heat_loss_map.len() - 1, heat_loss_map[0].len() - 1),
    );
    println!("{}", heat_loss);

    let heat_loss = find_min_heat_loss(
        &heat_loss_map,
        4,
        10,
        (0, 0),
        (heat_loss_map.len() - 1, heat_loss_map[0].len() - 1),
    );
    println!("{}", heat_loss);
}
