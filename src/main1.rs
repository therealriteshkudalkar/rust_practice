// 1. Two Sum

use std::collections::{HashMap};

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

pub fn main1() {
    // nums = [2,7,11,15], target = 9
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    println!("nums: {}, target: {}, two_sum: {}", format!("{:?}", nums), target,
             format!("{:?}", two_sum(nums, target)));

    // nums = [3,2,4], target = 6
    let nums = vec![3, 2, 4];
    let target = 6;
    println!("nums: {}, target: {}, two_sum: {}", format!("{:?}", nums), target,
             format!("{:?}", two_sum(nums, target)));
}

