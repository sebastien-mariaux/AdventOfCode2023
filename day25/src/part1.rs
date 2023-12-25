use crate::utils::read_data;
use petgraph::graph::Graph;
use rustworkx_core::centrality::edge_betweenness_centrality;
use rustworkx_core::petgraph;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn solve_puzzle(file_name: &str) -> u32 {
    let data = read_data(file_name);

    let mut nodes = HashSet::new();
    let mut connections = HashSet::new();

    data.lines().for_each(|line| {
        let (source, targets_str) = line.split_once(": ").unwrap();
        let targets = targets_str
            .split(' ')
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        nodes.insert(source.to_string());
        for target in targets.iter() {
            nodes.insert(target.to_string());
            let mut pair = [source, target];
            pair.sort();

            connections.insert((pair[0].to_string(), pair[1].to_string()));
        }
    });

    let mut graph_nodes = HashMap::new();

    let mut graph = Graph::new_undirected();
    for node in nodes.iter() {
        let node_index = graph.add_node(node);
        graph_nodes.insert(node, node_index);
    }
    for connection in connections.iter() {
        let source = graph_nodes.get(&connection.0).unwrap();
        let target = graph_nodes.get(&connection.1).unwrap();
        graph.add_edge(*source, *target, 1);
    }

    let output = edge_betweenness_centrality(&graph, false, 50);

    let mut mapp = connections
        .iter()
        .zip(output.iter())
        .map(|(x, y)| (x.0.to_string(), x.1.to_string(), y.unwrap()))
        .collect::<Vec<(String, String, f64)>>();
    mapp.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    let central_edges = mapp
        .iter()
        .rev()
        .take(3)
        .collect::<Vec<&(String, String, f64)>>();
    for central_edge in central_edges.iter() {
        connections.remove(&(central_edge.0.to_string(), central_edge.1.to_string()));
    }

    let group_1_leader = central_edges[0].0.to_string();
    let group_2_leader = central_edges[0].1.to_string();

    let group_1 = get_group(&connections, &group_1_leader);
    let group_2 = get_group(&connections, &group_2_leader);

    group_1.len() as u32 * group_2.len() as u32
}

fn get_group(connections: &HashSet<(String, String)>, leader: &String) -> HashSet<String> {
    let mut group = HashSet::new();
    group.insert(leader.to_string());

    let mut visited = HashSet::new();
    let mut stack = vec![leader.to_string()];
    while let Some(node) = stack.pop() {
        let current_connections = connections
            .iter()
            .filter(|(a, b)| a == &node || b == &node)
            .collect::<Vec<&(String, String)>>();
        for (node1, node2) in current_connections.iter() {
            group.insert(node1.to_string());
            group.insert(node2.to_string());
            if !visited.contains(node1) {
                stack.push(node1.to_string());
            }
            if !visited.contains(node2) {
                stack.push(node2.to_string());
            }
            visited.insert(node1.to_string());
            visited.insert(node2.to_string());
        }
    }
    group
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(54, solve_puzzle("test_data"));
    }

    #[test]
    fn test_solution() {
        assert_eq!(562772, solve_puzzle("input"));
    }
}
