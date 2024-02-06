
fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut result = vec![0; nums.len()];
    let mut product = 1;
    let mut has_zero_count = 0;
    for i in 0..nums.len() {
        if nums[i] == 0 {
            has_zero_count += 1;
            continue;
        }
        product = product * nums[i];
    }
    if has_zero_count == nums.len() {
        return result;
    }
    for i in 0..nums.len() {
        result[i] = if has_zero_count > 0 {
            if has_zero_count > 1 {
                0
            } else {
                // has zero count equal to 1
                if nums[i] == 0 {
                    product
                } else {
                    0
                }
            }
        } else {
            product / nums[i]
        };
    }
    return result;
}

pub fn main39() {
    let nums = vec![1, 2, 3, 4];
    println!("nums: {:?}; result: {:?}", nums, product_except_self(nums.clone()));

    let nums = vec![-1, 1, 0, -3, 3];
    println!("nums: {:?}; result: {:?}", nums, product_except_self(nums.clone()));

    let nums = vec![0, 0];
    println!("nums: {:?}; result: {:?}", nums, product_except_self(nums.clone()));

    let nums = vec![0, 4, 0];
    println!("nums: {:?}; result: {:?}", nums, product_except_self(nums.clone()));
}