// 1. Two Sum

use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<usize> {
    // declare a map
    let mut num_map: HashMap<i32, usize> = HashMap::new();
    for (index, num) in nums.iter().enumerate() {
        let difference = target - num;
        if num_map.contains_key(&difference) {
            let index1 = num_map.get(&difference).unwrap();
            // get the index of that key
            return vec![*index1, index];
        }
        num_map.insert(*num, index);
    }
    return vec![];
}
