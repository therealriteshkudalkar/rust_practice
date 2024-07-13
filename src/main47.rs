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

pub fn main47() {
    let nums = vec![2, 3, 4];
    println!(
        "nums: {:?}; earned: {}",
        nums,
        delete_and_earn(nums.clone())
    );

    let nums = vec![2, 2, 3, 3, 3, 4];
    println!(
        "nums: {:?}; earned: {}",
        nums,
        delete_and_earn(nums.clone())
    );

    let nums = vec![3, 1];
    println!(
        "nums: {:?}; earned: {}",
        nums,
        delete_and_earn(nums.clone())
    );

    let nums = vec![
        10, 8, 4, 2, 1, 3, 4, 8, 2, 9, 10, 4, 8, 5, 9, 1, 5, 1, 6, 8, 1, 1, 6, 7, 8, 9, 1, 7, 6, 8,
        4, 5, 4, 1, 5, 9, 8, 6, 10, 6, 4, 3, 8, 4, 10, 8, 8, 10, 6, 4, 4, 4, 9, 6, 9, 10, 7, 1, 5,
        3, 4, 4, 8, 1, 1, 2, 1, 4, 1, 1, 4, 9, 4, 7, 1, 5, 1, 10, 3, 5, 10, 3, 10, 2, 1, 10, 4, 1,
        1, 4, 1, 2, 10, 9, 7, 10, 1, 2, 7, 5,
    ];
    println!(
        "nums: {:?}; earned: {}",
        nums,
        delete_and_earn(nums.clone())
    );

    let nums = vec![
        12, 32, 93, 17, 100, 72, 40, 71, 37, 92, 58, 34, 29, 78, 11, 84, 77, 90, 92, 35, 12, 5, 27,
        92, 91, 23, 65, 91, 85, 14, 42, 28, 80, 85, 38, 71, 62, 82, 66, 3, 33, 33, 55, 60, 48, 78,
        63, 11, 20, 51, 78, 42, 37, 21, 100, 13, 60, 57, 91, 53, 49, 15, 45, 19, 51, 2, 96, 22, 32,
        2, 46, 62, 58, 11, 29, 6, 74, 38, 70, 97, 4, 22, 76, 19, 1, 90, 63, 55, 64, 44, 90, 51, 36,
        16, 65, 95, 64, 59, 53, 93,
    ];
    println!(
        "nums: {:?}; earned: {}",
        nums,
        delete_and_earn(nums.clone())
    );
}
