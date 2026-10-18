fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let mut largest = i32::MIN;
    for &candy in candies.iter() {
        if candy > largest {
            largest = candy;
        }
    }
    let mut bool_vec = vec![false; candies.len()];
    for (index, &candy) in candies.iter().enumerate() {
        if candy + extra_candies >= largest {
            bool_vec[index] = true;
        }
    }
    return bool_vec;
}
