use std::cmp::max;

fn candy(ratings: Vec<i32>) -> i32 {
    let n = ratings.len();
    if n == 1 {
        return 1;
    }
    let mut markings = vec![1; n];
    let mut curr_num = ratings[0];
    for i in 1..n {
        // go forward
        if curr_num < ratings[i] {
            // add 1 to sum and mark ratings[i]
            markings[i] = 1 + markings[i - 1];
        }
        curr_num = ratings[i];
    }
    curr_num = ratings[n - 1];
    for i in (0..=n-2).rev() {
        // go backwards
        if curr_num < ratings[i] {
            markings[i] = max(markings[i], 1 + markings[i + 1]);
        }
        curr_num = ratings[i].abs();
    }
    return markings.iter().sum();
}

pub fn main58() {
    let ratings = vec![1, 0, 2];
    assert_eq!(5, candy(ratings));

    let ratings = vec![1, 2, 2];
    assert_eq!(4, candy(ratings));

    let ratings = vec![1, 2, 2, 3, 2, 4];
    assert_eq!(9, candy(ratings));

    let ratings = vec![1, 2, 87, 87, 87, 2, 1];
    assert_eq!(13, candy(ratings));
}