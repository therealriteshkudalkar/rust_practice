use std::collections::{HashMap, HashSet};

fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
    // Form the hashmap
    let mut node_map: HashMap<i32, HashSet<i32>> = HashMap::new();

    for i in 1..=n {
        node_map.insert(i, HashSet::new());
    }

    for trust_rel in trust {
        let person_a = trust_rel[0];
        let person_b = trust_rel[1];
        if node_map.contains_key(&person_a) {
            node_map.get_mut(&person_a).unwrap().insert(person_b);
        } else {
            return -1;
        }
    }

    // Perform topological sort
    loop {
        let mut nodes_with_out_degree_zero: HashSet<i32> = HashSet::new();
        // Find the nodes with in-degree zero
        for (node, adj_list) in &node_map {
            if adj_list.len() == 0 {
                nodes_with_out_degree_zero.insert(*node);
            }
        }
        return if nodes_with_out_degree_zero.len() == 1 {
            let judge = *nodes_with_out_degree_zero.iter().next().unwrap();
            // Check if this is present in all the other hashsets
            let mut is_trust_worthy = true;
            for (node, adj_list) in &node_map {
                if *node != judge {
                    if !adj_list.contains(&judge) {
                        is_trust_worthy = false;
                    }
                }
            }
            return if is_trust_worthy {
                judge
            } else {
                -1
            }
        } else {
            -1
        }
    }
}

pub fn main44() {
    let n = 2;
    let trust = vec![vec![1, 2]];
    println!("n: {n}; trust: {:?}; judge: {}", trust, find_judge(n, trust.clone()));

    let n = 3;
    let trust = vec![vec![1, 3], vec![2, 3]];
    println!("n: {n}; trust: {:?}; judge: {}", trust, find_judge(n, trust.clone()));

    let n = 2;
    let trust = vec![vec![1, 3], vec![2, 3], vec![3, 1]];
    println!("n: {n}; trust: {:?}; judge: {}", trust, find_judge(n, trust.clone()))
}