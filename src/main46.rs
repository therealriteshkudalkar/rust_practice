use std::cmp::max;
use std::collections::HashMap;

fn rob_houses_memoized(
    nums: &Vec<i32>,
    memo: &mut HashMap<String, i32>,
    curr_house: usize,
    prev_selected: bool,
) -> i32 {
    let key = format!("{curr_house},{prev_selected}");
    if memo.contains_key(&key) {
        return memo[&key];
    }

    if curr_house >= nums.len() {
        return 0;
    }
    let max_sum = if prev_selected {
        rob_houses_memoized(nums, memo, curr_house + 1, false)
    } else {
        max(
            nums[curr_house] + rob_houses_memoized(nums, memo, curr_house + 1, true),
            rob_houses_memoized(nums, memo, curr_house + 1, false),
        )
    };
    memo.insert(key, max_sum);
    return max_sum;
}

fn rob_houses(nums: &Vec<i32>, curr_house: usize, prev_selected: bool) -> i32 {
    if curr_house >= nums.len() {
        return 0;
    }

    // choose
    return if prev_selected {
        // only consider the case where the current one is not robbed
        rob_houses(nums, curr_house + 1, false)
    } else {
        // consider both cases where the current one is robber or not robbed
        max(
            nums[curr_house] + rob_houses(nums, curr_house + 1, true),
            rob_houses(nums, curr_house + 1, false),
        )
    };
}

fn rob(nums: Vec<i32>) -> i32 {
    //return rob_houses(&nums, 0, false);
    return rob_houses_memoized(&nums, &mut HashMap::new(), 0, false);
}

pub fn main46() {
    let nums = vec![1, 2, 3, 1];
    println!("house_stash: {:?}; max_value: {}", nums, rob(nums.clone()));

    let nums = vec![2, 7, 9, 3, 1];
    println!("house_stash: {:?}; max_value: {}", nums, rob(nums.clone()));

    let nums = vec![2, 1, 1, 2];
    println!("house_stash: {:?}; max_value: {}", nums, rob(nums.clone()));
}
