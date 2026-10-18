use std::collections::HashSet;

fn permutation_with_duplicate_numbers(nums: &Vec<i32>, set: &mut HashSet<usize>,
                                      collection: &mut HashSet<Vec<i32>>, permutation: &mut Vec<i32>) {
    if permutation.len() == nums.len() {
        collection.insert(permutation.clone());
    }
    for (index, num) in nums.iter().enumerate() {
        if !set.contains(&index) {
            set.insert(index);
            permutation.push(*num);
            permutation_with_duplicate_numbers(nums, set, collection, permutation);
            permutation.pop();
            set.remove(&index);
        }
    }
}

fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut collection: HashSet<Vec<i32>> = HashSet::new();
    permutation_with_duplicate_numbers(&nums, &mut HashSet::new(), &mut collection, &mut Vec::new());
    return collection.into_iter().collect();
}
