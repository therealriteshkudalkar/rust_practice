use std::cmp::min;

pub fn min_cost_climbing_stairs(cost: &Vec<i32>) -> i32 {
    let n = cost.len();
    let mut min_cost_ar = vec![0; n + 1];
    min_cost_ar[0] = 0;
    min_cost_ar[1] = 0;

    // generate minimum cost array for each index
    for i in 2..=n {
        // get the min of i - 1 and i - 2
        min_cost_ar[i] = min(cost[i - 1] + min_cost_ar[i - 1],
                             cost[i - 2] + min_cost_ar[i - 2]);
    }
    return min_cost_ar[n];
}

pub fn main12() {
    let cost = vec![3, 3, 7, 9];
    println!("cost: {:?}; min_cost_climbing_stairs: {}", cost, min_cost_climbing_stairs(&cost));
    let cost = vec![10, 15, 20];
    println!("cost: {:?}; min_cost_climbing_stairs: {}", cost, min_cost_climbing_stairs(&cost));
    let cost = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
    println!("cost: {:?}; min_cost_climbing_stairs: {}", cost, min_cost_climbing_stairs(&cost));
}