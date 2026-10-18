use std::cmp::max;
use std::collections::HashSet;

fn depth_first_search(adj_list: &Vec<Vec<usize>>, scores: &Vec<i32>, stack: &mut HashSet<usize>,
                      source: usize) -> i32 {
    stack.insert(source);
    if stack.len() == 4 {
        stack.remove(&source);
        return scores[source];
    }
    let mut max_score = -1;
    for node in adj_list[source].iter() {
        // push that node in the stack
        if !stack.contains(node) {
            let score = depth_first_search(adj_list, scores, stack, *node);
            if score != -1 {
                max_score = max(max_score, scores[*node] + score);
            }
            stack.remove(node);
        }
    }
    stack.remove(&source);
    return max_score;
}

fn maximum_score(scores: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
    // Calculate the Adjacency List for the given edges
    let n = scores.len();
    let mut adj_list: Vec<Vec<usize>> = vec![Vec::new(); n];
    for edge in edges.iter() {
        let first_node = edge[0] as usize;
        let second_node = edge[1] as usize;
        adj_list[first_node ].push(second_node);
        adj_list[second_node].push(first_node);
    }
    let mut max_score = -1;
    for source in 0..n {
        // For each source find the max_score
        max_score = max(max_score, depth_first_search(&mut adj_list, &scores, &mut HashSet::new(), source));
    }
    return max_score;
}
