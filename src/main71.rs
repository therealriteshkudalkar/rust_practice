use std::collections::BinaryHeap;

fn max_product(nums: Vec<i32>) -> i32 {
    let mut max_heap = BinaryHeap::new();
    for num in nums.iter() {
        max_heap.push(*num);
    }
    (max_heap.pop().unwrap() - 1) * (max_heap.pop().unwrap() - 1)
}

pub fn main71() {
    assert_eq!(12, max_product(vec![3, 4, 5, 2]));
    
    assert_eq!(16, max_product(vec![1, 5, 4, 5]));
    
    assert_eq!(12, max_product(vec![3, 7]));
}