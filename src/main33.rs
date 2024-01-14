use std::collections::HashMap;

// This takes O(N) space and O(N) time
fn majority_element(nums: Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut maj_el = nums[0];
    let ceil = nums.len() as i32 / 2i32 + 1;
    for num in nums {
        if map.contains_key(&num) {
            let new_count = *map.get(&num).unwrap() + 1;
            map.insert(num, new_count);
            if new_count == ceil {
                return num;
            }
        } else {
            map.insert(num, 1);
        }
    }
    return maj_el;
}

// This takes O(1) space and O(N) time
fn majority_element_optimized(nums: Vec<i32>) -> i32 {
    let mut bytes = [false; 32];
    let n = nums.len();
    for i in 0..32 {
        let mut ones_count = 0;
        for num in &nums {
            if (1 << i & *num) >> i == 1 {
                ones_count += 1;
            }
        }
        bytes[i] = ones_count > n / 2;
    }
    let mut num = 0;
    for i in 0..32 {
        num += if bytes[i] { 1 << i } else { 0 };
    }
    return num;
}

pub fn main33() {
    let vec1 = vec![3, 2, 3];
    println!("vec: {:?}; Majority element: {}", vec1, majority_element(vec1.clone()));

    let vec1 = vec![2, 2, 1, 1, 1, 2, 2];
    println!("vec: {:?}; Majority element: {}", vec1, majority_element(vec1.clone()));

    let vec1 = vec![3, 2, 3];
    println!("vec: {:?}; Majority element: {}", vec1, majority_element_optimized(vec1.clone()));

    let vec1 = vec![2, 2, 1, 1, 1, 2, 2];
    println!("vec: {:?}; Majority element: {}", vec1, majority_element_optimized(vec1.clone()));
}