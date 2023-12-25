use std::{collections::HashMap, fs::File, io::Write};

const INPUT: &str = include_str!("input.txt");

fn remove_edge(graph: &mut [Vec<usize>], source: usize, target: usize) {
    graph[source].retain(|x| *x != target);
    graph[target].retain(|x| *x != source);
}

fn get_components_size(graph: &[Vec<usize>]) -> Vec<usize> {
    let mut is_visited = vec![false; graph.len()];
    let mut components: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut stack: Vec<usize> = vec![];
    loop {
        let Some(start) = is_visited.iter().position(|x| !*x) else {
            break;
        };
        components.insert(start, vec![]);
        stack.clear();
        stack.push(start);
        while let Some(node) = stack.pop() {
            if is_visited[node] {
                continue;
            }
            components.get_mut(&start).unwrap().push(node);
            is_visited[node] = true;
            for &next in &graph[node] {
                stack.push(next);
            }
        }
    }
    components.values().map(|x| x.len()).collect()
}

fn write_dot_graph(graph: &[Vec<usize>]) {
    let mut graph_file = File::create("graph.dot").unwrap();
    graph_file.write(b"graph G {\n").unwrap();
    for (source, targets) in graph.iter().enumerate() {
        graph_file
            .write(
                format!(
                    "{} -- {{ {} }}\n",
                    source,
                    targets
                        .iter()
                        .filter(|&&x| x > source)
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(" ")
                )
                .as_bytes(),
            )
            .unwrap();
    }
    graph_file.write(b"}\n").unwrap();
}

fn main() {
    let mut lookup_table: Vec<String> = vec![];
    let mut graph: Vec<Vec<usize>> = vec![];
    for line in INPUT.lines() {
        let (source, targets) = line.split_once(": ").unwrap();
        let source_node = match lookup_table.iter().position(|x| x == source) {
            Some(id) => id,
            None => {
                lookup_table.push(source.to_string());
                lookup_table.len() - 1
            }
        };
        for target in targets.split(' ') {
            let target_node = match lookup_table.iter().position(|x| x == target) {
                Some(id) => id,
                None => {
                    lookup_table.push(target.to_string());
                    lookup_table.len() - 1
                }
            };
            if graph.len() < lookup_table.len() {
                graph.resize(lookup_table.len(), vec![]);
            }
            graph[source_node].push(target_node);
            graph[target_node].push(source_node);
        }
    }
    write_dot_graph(&graph);
    remove_edge(&mut graph, 16, 18);
    assert_eq!(get_components_size(&graph).len(), 1);
    remove_edge(&mut graph, 1108, 1446);
    assert_eq!(get_components_size(&graph).len(), 1);
    remove_edge(&mut graph, 1082, 610);
    let components = get_components_size(&graph);
    assert_eq!(components.len(), 2);
    println!("{:?}", components.iter().product::<usize>());
}
