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
