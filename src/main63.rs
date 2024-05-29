fn permutation_without_repetition(nums: &Vec<i32>, collection: &mut Vec<Vec<i32>>,
                                  permutation: &mut Vec<i32>) {
    if permutation.len() == nums.len() {
        collection.push(permutation.clone());
    }
    for num in nums.iter() {
        if !permutation.contains(num) {
            permutation.push(*num);
            permutation_without_repetition(nums, collection, permutation);
            permutation.pop();
        }
    }
}

fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut collection = Vec::new();
    permutation_without_repetition(&nums, &mut collection, &mut Vec::new());
    return collection;
}

pub fn main63() {
    let nums = vec![1, 2, 3];
    assert_eq!(
        vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1]
        ],
        permute(nums)
    );

    let nums = vec![1, 2];
    assert_eq!(
        vec![
            vec![1, 2],
            vec![2, 1]
        ],
        permute(nums)
    );

    let nums = vec![1];
    assert_eq!(
        vec![vec![1]],
        permute(nums)
    );
}
