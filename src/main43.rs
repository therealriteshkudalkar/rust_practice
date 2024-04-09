use std::collections::{HashMap, HashSet};

fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
    // Create a hashmap of key having node and value is hashset of adjacency list
    let mut node_map = HashMap::new();
    for (node, adj_list) in graph.iter().enumerate() {
        let mut set = HashSet::new();
        for adj_node in adj_list {
            set.insert(*adj_node);
        }
        node_map.insert(node, set);
    }

    // Topological sorting
    let mut has_in_degree_zero_nodes = true;
    let mut safe_nodes = HashSet::new();
    while has_in_degree_zero_nodes {
        // Identify nodes with in-degree zero
        let mut in_degree_zero_nodes = HashSet::new();
        for (node, adj_list) in &node_map {
            if adj_list.len() == 0 {
                in_degree_zero_nodes.insert(*node as i32);
            }
        }
        if in_degree_zero_nodes.len() == 0 {
            has_in_degree_zero_nodes = false;
            break;
        }

        // Remove in-degree zero nodes from the map
        for node in &in_degree_zero_nodes {
            node_map.remove(&(*node as usize));
            safe_nodes.insert(*node);
        }
        // Remove in-degree zero nodes from the adjacency list of other nodes
        let mut node_keys= Vec::new();
        for node in node_map.keys() {
            node_keys.push(*node);
        }
        for node in node_keys {
            // Find the set difference
            let difference: HashSet<i32> = node_map.get(&node).unwrap()
                .difference(&in_degree_zero_nodes).cloned().collect();
            node_map.insert(node, difference);
        }
    }
    let mut return_vec: Vec<i32> = Vec::new();
    for node in safe_nodes {
        return_vec.push(node);
    }
    return_vec.sort();
    return return_vec;
}

pub fn main43() {
    let graph = vec![vec![1, 2], vec![2, 3], vec![5], vec![0], vec![5], vec![], vec![]];
    println!("Graph: {:?}; Eventual Safe Nodes: {:?}", graph, eventual_safe_nodes(graph.clone()))
}