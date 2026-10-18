fn missing_number(nums: Vec<i32>) -> i32 {
    let n = nums.len() as i32;
    let sum: i32 = n * (n + 1) / 2;
    let actual_sum: i32 = nums.iter().sum();
    return sum - actual_sum;
}

fn missing_number_with_xor_trick(nums: Vec<i32>) -> i32 {
    let mut missing_number = 0;
    for i in 0..nums.len() {
        missing_number ^= nums[i];
        missing_number ^= i as i32;
    }
    missing_number ^= nums.len() as i32;
    return missing_number;
}
