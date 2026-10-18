use std::cmp::max;
use std::collections::{HashMap, HashSet};

fn delete_and_earn_process(nums: Vec<i32>) -> i32 {
    // Create a Hashmap with the keys sorted in ascending order.
    let mut num_count: HashMap<i32, i32> = HashMap::new();
    for num in nums {
        match num_count.get(&num) {
            None => {
                num_count.insert(num, 1);
            }
            Some(count) => {
                num_count.insert(num, count + 1);
            }
        }
    }
    // Get the keys of the map and sort them in ascending order
    let mut keys: Vec<i32> = num_count.keys().cloned().collect();
    keys.sort();

    // Pass these keys
    //return delete_and_earn_with_rec(&num_count, &keys, &mut HashSet::new(), 0);
    return delete_and_earn_with_memoized(&mut HashMap::new(), &num_count, &keys, &mut HashSet::new(),0);
}

#[allow(dead_code)]
fn delete_and_earn_with_rec(
    nums: &HashMap<i32, i32>,
    order: &Vec<i32>,
    selected: &mut HashSet<i32>,
    current_index: usize
) -> i32 {
    if current_index >= order.len() {
        return 0;
    }
    // declare a Hashset which contains which one is taken and which one is not
    let curr_val = order[current_index];
    let curr_val_count = *nums.get(&curr_val).unwrap();

    return if selected.contains(&(curr_val + 1)) || selected.contains(&(curr_val - 1)) {
        // skip the current index
        delete_and_earn_with_rec(nums, order, selected, current_index + 1)
    } else {
        selected.insert(curr_val);
        let val_1 = curr_val * curr_val_count + delete_and_earn_with_rec(nums, order, selected, current_index + 1);
        selected.remove(&curr_val);
        let val_2 = delete_and_earn_with_rec(nums, order, selected, current_index + 1);
        max(val_1, val_2)
    }
}

fn delete_and_earn_with_memoized(
    memo: &mut HashMap<String, i32>,
    nums: &HashMap<i32, i32>,
    order: &Vec<i32>,
    selected: &mut HashSet<i32>,
    current_index: usize
) -> i32 {
    let map_key = format!("{current_index}");
    if memo.contains_key(&map_key) {
        return *memo.get(&map_key).unwrap();
    }
    if current_index >= order.len() {
        return 0;
    }

    let curr_val = order[current_index];
    let curr_val_count = *nums.get(&curr_val).unwrap();

    let max_val = if selected.contains(&(curr_val + 1)) || selected.contains(&(curr_val - 1)) {
         delete_and_earn_with_memoized(memo, nums, order, selected, current_index + 1)
    } else {
        selected.insert(curr_val);
        let val_1 = curr_val * curr_val_count + delete_and_earn_with_memoized(memo, nums, order, selected, current_index + 1);
        selected.remove(&curr_val);
        let val_2 = delete_and_earn_with_memoized(memo, nums, order, selected, current_index + 1);
        max(val_1, val_2)
    };
    memo.insert(map_key, max_val);
    return max_val;
}

#[allow(dead_code)]
fn delete_and_earn_memoized(
    nums: &Vec<i32>,
    memo: &mut HashMap<String, i32>,
    curr_index: usize,
    hs: &mut HashSet<i32>,
) -> i32 {
    let key = format!("{curr_index},{:?}", hs);
    if memo.contains_key(&key) {
        return memo[&key];
    }
    if curr_index >= nums.len() {
        return 0;
    }
    let curr_val = nums[curr_index];
    let max_value = if hs.contains(&curr_val) {
        curr_val + delete_and_earn_memoized(nums, memo, curr_index + 1, hs)
    } else {
        if hs.contains(&(curr_val + 1)) || hs.contains(&(curr_val - 1)) {
            delete_and_earn_memoized(nums, memo, curr_index + 1, hs)
        } else {
            hs.insert(curr_val);
            let val_1 = curr_val + delete_and_earn_memoized(nums, memo, curr_index + 1, hs);
            hs.remove(&curr_val);
            let val_2 = delete_and_earn_memoized(nums, memo, curr_index + 1, hs);
            max(val_1, val_2)
        }
    };
    memo.insert(key, max_value);
    return max_value;
}

#[allow(dead_code)]
fn delete_and_earn_rec(nums: &Vec<i32>, curr_index: usize, hs: &mut HashSet<i32>) -> i32 {
    if curr_index >= nums.len() {
        return 0;
    }
    let curr_val = nums[curr_index];
    let max_value = if hs.contains(&curr_val) {
        // always delete if already present in the hashmap
        curr_val + delete_and_earn_rec(nums, curr_index + 1, hs)
    } else {
        // doesn't contain then
        if hs.contains(&(curr_val - 1)) || hs.contains(&(curr_val + 1)) {
            // auto delete
            delete_and_earn_rec(nums, curr_index + 1, hs)
        } else {
            hs.insert(curr_val);
            let val_1 = curr_val + delete_and_earn_rec(nums, curr_index + 1, hs);
            hs.remove(&curr_val);
            let val_2 = delete_and_earn_rec(nums, curr_index + 1, hs);
            max(val_1, val_2)
        }
    };
    return max_value;
}

fn delete_and_earn(nums: Vec<i32>) -> i32 {
    //return delete_and_earn_memoized(&nums, &mut HashMap::new(), 0, &mut HashSet::new());
    return delete_and_earn_process(nums)
}
