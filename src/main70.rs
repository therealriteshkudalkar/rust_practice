use std::collections::BinaryHeap;

#[derive(Debug, PartialOrd, PartialEq, Eq, Ord)]
struct Person(i32, usize);

pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
    let mut max_heap = BinaryHeap::new();
    for (index, person_score) in score.iter().enumerate() {
        max_heap.push(Person(*person_score, index));
    }
    let mut count = 1;
    let mut result: Vec<String> = vec![String::new(); score.len()];
    while !max_heap.is_empty() {
        let person = max_heap.pop().unwrap();
        match count {
            1 => result[person.1] = String::from("Gold Medal"),
            2 => result[person.1] = String::from("Silver Medal"),
            3 => result[person.1] = String::from("Bronze Medal"),
            _ => result[person.1] = format!("{count}"),
        }
        count += 1
    }
    result
}

pub fn main70() {
    assert_eq!(
        vec![
            String::from("Gold Medal"),
            String::from("Silver Medal"),
            String::from("Bronze Medal"),
            String::from("4"),
            String::from("5")
        ],
        find_relative_ranks(vec![5, 4, 3, 2, 1])
    );

    assert_eq!(
        vec![
            String::from("Gold Medal"),
            String::from("5"),
            String::from("Bronze Medal"),
            String::from("Silver Medal"),
            String::from("4"),
        ],
        find_relative_ranks(vec![10, 3, 8, 9, 4])
    );
}