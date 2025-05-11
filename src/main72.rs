use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Pair(i32, usize);

fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut min_heap = BinaryHeap::new();
    for (index, num) in nums.iter().enumerate() {
        // Push if it is less than k and if not then check if main
        if min_heap.len() < k as usize {
            min_heap.push(Reverse(Pair(*num, index)));
        } else {
            match min_heap.peek() {
                None => { min_heap.push(Reverse(Pair(*num, index))) }
                Some(pair) => {
                    if pair.0.0 < *num {
                        min_heap.pop();
                        min_heap.push(Reverse(Pair(*num, index)));
                    }
                }
            }
        }
    }
    // Sort the pairs according to indices
    let mut max_items: Vec<Pair> = Vec::new();
    while !min_heap.is_empty() {
        max_items.push(min_heap.pop().unwrap().0);
    }
    max_items.sort_by(|pair_a, pair_b| pair_a.1.cmp(&pair_b.1));
    max_items.iter().map(|pair_a| pair_a.0).collect()
}

pub fn main72() {
    assert_eq!(vec![3, 3], max_subsequence(vec![2, 1, 3, 3], 2));

    assert_eq!(vec![-1, 3, 4], max_subsequence(vec![-1, -2, 3, 4], 3));

    assert_eq!(vec![3, 4], max_subsequence(vec![3, 4, 3, 3], 2));
}
