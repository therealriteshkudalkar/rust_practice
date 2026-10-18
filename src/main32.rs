fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut k = 1;
    let mut prev_element = nums[0];
    let mut prev_element_count = 1;
    for i in 1..nums.len() {
        if nums[i] != prev_element {
            prev_element = nums[i];
            prev_element_count = 1;
            nums[k] = nums[i];
            k += 1;
        } else {
            if prev_element_count < 2 {
                prev_element_count += 1;
                nums[k] = nums[i];
                k += 1;
            }
        }
    }
    return k as i32;
}
