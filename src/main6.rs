
#[allow(dead_code)]
fn can_be_increasing_fast(nums: &Vec<i32>) -> bool {
    let mut count_of_rule_breakers = 0;
    for i in 0..nums.len()-1 {
        // check if i < i + 1
        if nums[i] >= nums[i + 1] {
            count_of_rule_breakers += 1;
            // also check if nums[i - 1] >= nums[i + 1]
            if i >= 1 && nums[i - 1] >= nums[i + 1] && i != nums.len() - 2 {
                count_of_rule_breakers += 1;
            }
            // also check if nums[i] < nums[i + 2]
            if i + 2 < nums.len() && nums[i] < nums[i + 2] {
                // the culprit is (i+1)th index
                count_of_rule_breakers -= 1;
            }
        }
    }
    println!("{count_of_rule_breakers}");
    count_of_rule_breakers <= 1
}

fn is_increasing(nums: &Vec<i32>, skip: usize) -> bool {
    for i in 0..nums.len() - 1 {
        if i == skip {
            continue;
        } else if i + 1 == skip {
            if i + 2 < nums.len() && nums[i] >= nums[i + 2] {
                return false;
            }
        } else if nums[i] >= nums[i + 1] {
            return false;
        }
    }
    true
}

fn can_be_increasing(nums: &Vec<i32>) -> bool {
    for i in 0..nums.len() {
        if is_increasing(nums, i) {
            return true;
        }
    }
    false
}
