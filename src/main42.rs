use std::collections::{HashSet};

fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
    // Traverse through each component
    let mut remaining_nodes = HashSet::new();
    for i in 0..is_connected.len() {
        remaining_nodes.insert(i);
    }
    let mut count = 0;
    while !remaining_nodes.is_empty() {
        count += 1;

        // pick a source node
        let source = *remaining_nodes.iter().next().unwrap();
        remaining_nodes.remove(&source);

        //Apply BFS or DFS
        let mut visited_nodes = HashSet::new();
        let mut nodes_in_stack = HashSet::new();
        let mut stack = Vec::new();
        stack.push(source);
        nodes_in_stack.insert(source);
        while !stack.is_empty() {
            // go through its adjacency list and check if any of them are not visited
            // if they are not visited then append them in the queue
            let examined_element = stack.pop().unwrap();
            nodes_in_stack.remove(&examined_element);
            for (adj_element, is_conn) in is_connected[examined_element].iter().enumerate() {
                if *is_conn != 0 {
                    if visited_nodes.contains(&adj_element) || nodes_in_stack.contains(&adj_element) || adj_element == examined_element {
                        continue
                    }
                    stack.push(adj_element);
                    nodes_in_stack.insert(adj_element);
                }
            }
            visited_nodes.insert(examined_element);
        }
        for node in visited_nodes {
            remaining_nodes.remove(&node);
        }
    }

    return count;
}

pub fn main42() {
    // Test cases go here
    let is_connected_vec = vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]];
    println!("is_connected_vec: {:?}; provinces: {:?}", is_connected_vec,
             find_circle_num(is_connected_vec.clone()));

    let is_connected_vec = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
    println!("is_connected_vec: {:?}; provinces: {:?}", is_connected_vec,
             find_circle_num(is_connected_vec.clone()));

    let is_connected_vec = vec![vec![1, 0, 0, 1], vec![0, 1, 1, 0], vec![0, 1, 1, 1],
                                vec![1, 0, 1, 1]];
    println!("is_connected_vec: {:?}; provinces: {:?}", is_connected_vec,
             find_circle_num(is_connected_vec.clone()));
}