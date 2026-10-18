use std::collections::BinaryHeap;

fn max_product(nums: Vec<i32>) -> i32 {
    let mut max_heap = BinaryHeap::new();
    for num in nums.iter() {
        max_heap.push(*num);
    }
    (max_heap.pop().unwrap() - 1) * (max_heap.pop().unwrap() - 1)
}
