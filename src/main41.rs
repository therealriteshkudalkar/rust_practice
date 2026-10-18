use std::collections::{HashMap, HashSet};

#[allow(dead_code)]
fn three_sum_naive(nums: Vec<i32>) -> Vec<Vec<i32>>{
    let mut set: HashSet<Vec<i32>> = HashSet::new();
    for (i, &num) in nums.iter().enumerate() {
        let target = -num;
        let mut map = HashMap::new();
        for (j, &num1) in nums.iter().enumerate() {
            // find the two sum solution for num
            let diff = target - num1;
            if map.contains_key(&diff) {
                let k = map.get(&diff).unwrap();
                if i != j && j != *k && *k != i {
                    let mut index_vec = vec![num, num1, nums[*k]];
                    index_vec.sort();
                    if !set.contains(&index_vec) {
                        set.insert(index_vec);
                    }
                }
            }
            map.insert(num1, j);
        }
    }
    set.into_iter().collect()

}

#[allow(dead_code)]
fn two_sum_fast(nums: &Vec<i32>, target: i32) -> Vec<usize> {
    // Find two numbers from num, where num[i]+num[j]=value
    let mut map: HashMap<i32, usize> = HashMap::new();
    for (index, num) in nums.iter().enumerate() {
        let difference = target - *num;
        if map.contains_key(&difference) {
            // return the indices of the item
            return vec![*map.get(&difference).unwrap(), index];
        }
        map.insert(*num, index);
    }
    vec![]
}

#[allow(dead_code)]
fn three_sum_faster(nums: Vec<i32>) -> Vec<Vec<i32>> {
    // Intuition: Extend the two sum solution for the three sum problem
    let mut set: HashSet<Vec<i32>> = HashSet::new();
    let mut map: HashMap<i32, Vec<usize>> = HashMap::new();
    let n = nums.len();
    for i in 0..n {
        let num = nums[i];
        let target = -num;
        if map.contains_key(&target) {
            // Don't compute the two sum again as the values will remain the same
            continue;
        }
        // Compute the two sum
        let indices = two_sum_fast(&nums, target);
        let indices_len = indices.len();
        map.insert(target, indices.clone());
        // Check if the length of the indices vector is greater than 0
        if indices_len == 0 {
            continue;
        }
        // If the length is not zero that means the target it found now just
        let index_item1_target = -nums[indices[0]];
        let index_item2_target = -nums[indices[1]];
        map.insert(index_item1_target, vec![indices[1], i]);
        map.insert(index_item2_target, vec![indices[0], i]);
        let mut resultant_vec = vec![num, nums[indices[0]], nums[indices[1]]];
        resultant_vec.sort();
        set.insert(resultant_vec);
    }
    set.into_iter().collect()
}

fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut sorted_nums: Vec<i32> = nums;
    sorted_nums.sort();
    let nums_len = sorted_nums.len();
    let mut solution_set: HashSet<Vec<i32>> = HashSet::new();
    for (index, &num) in sorted_nums.iter().enumerate() {
        if index > 0 && num == sorted_nums[index - 1] {
            continue;
        }
        let target = -num;
        let mut left = index + 1;
        let mut right = nums_len - 1;
        while left < right {
            // Check if they are equal
            let sum = sorted_nums[left] + sorted_nums[right];
            if sum == target {
                // Form the triplet
                let mut solution = vec![num, sorted_nums[left], sorted_nums[right]];
                solution.sort();
                solution_set.insert(solution);
                left += 1;
                right -= 1;
            } else if sum > target {
                // sum < target so move right more left
                right -= 1;
            } else {
                // sum < target so move left more right
                left += 1;
            }
        }
    }
    solution_set.into_iter().collect()
}
