fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    // Loop through the nums array
    let mut return_vec = Vec::new();

    return_vec
}

pub fn main65() {
    let nums = vec![1, 0, -1, 0, -2, 2];
    assert_eq!(
        four_sum(nums, 0),
        vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]]
    )
}
