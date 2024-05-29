fn find_duplicate_naive(nums: Vec<i32>) -> i32 {
    let mut duplicate_number = 0;
    let mut count = 0;
    for i in 0..nums.len() {
        if i > 0 && nums[i - 1] == nums[i] {
            count += 1;
        }
        duplicate_number ^= nums[i];
        duplicate_number ^= i as i32;
    }
    if count == nums.len() - 1 {
        return nums[0];
    }
    return duplicate_number;
}

fn find_duplicate_smarter(nums: Vec<i32>) -> i32 {
    let mut cloned_nums: Vec<i32> = nums.clone();
    cloned_nums.sort();
    for i in 1..cloned_nums.len() {
        if cloned_nums[i] == cloned_nums[i - 1] {
            return cloned_nums[i];
        }
    }
    return -1;
}

fn find_duplicate(nums: Vec<i32>) -> i32 {
    // 1 <= nums[i] <= n
    let mut nums = nums.clone();
    for i in 0..nums.len() {
        if nums[i] as usize != i {
            // if the xth element is not at xth index, then
            // check if xth index already has that element
            // // yes then this is the duplicate element
            // // no then swap the element
            if nums[i] == nums[nums[i] as usize] {
                return nums[i];
            } else {
                let temp_1 = nums[i];
                let temp_2 = nums[temp_1 as usize];
                nums[i] = temp_2;
                nums[temp_1 as usize] = temp_1;
            }
        }
    }
    return -1;
}

pub fn main53() {
    let nums = vec![1, 3, 4, 2, 2];
    println!(
        "nums: {:?}; duplicate_number: {}",
        nums,
        find_duplicate(nums.clone())
    );

    let nums = vec![3, 1, 3, 4, 2];
    println!(
        "nums: {:?}; duplicate_number: {}",
        nums,
        find_duplicate(nums.clone())
    );

    let nums = vec![3, 3, 3, 3, 3];
    println!(
        "nums: {:?}; duplicate_number: {}",
        nums,
        find_duplicate(nums.clone())
    );

    let nums = vec![1, 4, 4, 2, 4];
    println!(
        "nums: {:?}; duplicate_number: {}",
        nums,
        find_duplicate(nums.clone())
    );

    let nums = vec![1, 6, 3, 2, 1, 5, 4];
    println!(
        "nums: {:?}; duplicate_number: {}",
        nums,
        find_duplicate(nums.clone())
    );

    let nums = vec![1, 2, 2];
    println!(
        "nums: {:?}; duplicate_number: {}",
        nums,
        find_duplicate(nums.clone())
    );


    let nums = vec![1, 3, 4, 2, 2];
    println!(
        "nums: {:?}; duplicate_number: {}",
        nums,
        find_duplicate_naive(nums.clone())
    );

    let nums = vec![3, 1, 3, 4, 2];
    println!(
        "nums: {:?}; duplicate_number: {}",
        nums,
        find_duplicate_naive(nums.clone())
    );

    let nums = vec![3, 3, 3, 3, 3];
    println!(
        "nums: {:?}; duplicate_number: {}",
        nums,
        find_duplicate_naive(nums.clone())
    );

    let nums = vec![1, 4, 4, 2, 4];
    println!(
        "nums: {:?}; duplicate_number: {}",
        nums,
        find_duplicate_naive(nums.clone())
    );

    let nums = vec![1, 6, 3, 2, 1, 5, 4];
    println!(
        "nums: {:?}; duplicate_number: {}",
        nums,
        find_duplicate_naive(nums.clone())
    );

    let nums = vec![1, 2, 2];
    println!(
        "nums: {:?}; duplicate_number: {}",
        nums,
        find_duplicate_naive(nums.clone())
    );
}
