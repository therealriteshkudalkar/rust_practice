fn can_jump(nums: Vec<i32>) -> bool {
    // traverse from the other end of the array
    // mark indexes which are from the left of n - 1 as reach able
    let mut reachable_to_last_index = vec![false; nums.len()];
    reachable_to_last_index[nums.len() - 1] = true;
    for i in (0..nums.len()-1).rev() {
        let curr_num = nums[i];
        // check if this is reachable to some other reachable index
        if curr_num == 0 {
            reachable_to_last_index[i] = false;
        } else {
            // loop from current index to current_index + current_value
            let max = if i + curr_num as usize > nums.len() - 1 { nums.len() - 1 } else { i + curr_num as usize };
            let mut flag = false;
            for j in i + 1 ..= max {
                if reachable_to_last_index[j] {
                    flag = true;
                    break;
                }
            }
            reachable_to_last_index[i] = flag;
        }
    }
    return reachable_to_last_index[0];
}

pub fn main36() {
    let nums = vec![2, 3, 1, 1, 4];
    println!("nums: {:?}; can_jump: {}", nums, can_jump(nums.clone()));

    let nums = vec![3, 2, 1, 0, 4];
    println!("nums: {:?}; can_jump: {}", nums, can_jump(nums.clone()));

    let nums = vec![2, 2, 1, 0, 4];
    println!("nums: {:?}; can_jump: {}", nums, can_jump(nums.clone()));
}