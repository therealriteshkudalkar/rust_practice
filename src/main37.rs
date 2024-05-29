
fn jump(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut reachablility = vec![false; n];
    let mut least_step = vec![i32::MAX; n];
    least_step[n - 1] = 0;
    reachablility[n - 1] = true;
    for i in (0..n - 1).rev() {
        let max_val = if i + nums[i] as usize > n - 1 { n - 1 } else { i + nums[i] as usize };
        for j in i+1..=max_val {
            if reachablility[j] {
                reachablility[i] = true;
                // check how many hops it required
                if least_step[i] > least_step[j] + 1 {
                    least_step[i] = least_step[j] + 1;
                }
            }
        }
    }
    return least_step[0];
}

pub fn main37() {
    let nums = vec![2, 3, 1, 1, 4];
    println!("nums: {:?}; jumps: {}", nums, jump(nums.clone()));

    let nums = vec![2, 3, 0, 1, 4];
    println!("nums: {:?}; jumps: {}", nums, jump(nums.clone()));
}