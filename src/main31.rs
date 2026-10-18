fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut k = 1;
    let mut prev_element = nums[0];
    for i in 1..nums.len() {
        if nums[i] != prev_element {
            prev_element = nums[i];
            nums[k] = nums[i];
            k += 1;
        }
    }
    return k as i32;
}
