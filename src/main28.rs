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

pub fn main28() {
    let candies = vec![2, 3, 5, 1, 3];
    let extra_candies = 3;
    println!("extra_candies: {extra_candies}, candies: {:?}, kids_with_candies: {:?}",
             candies, kids_with_candies(candies.clone(), extra_candies));

    let candies = vec![4, 2, 1, 1, 2];
    let extra_candies = 1;
    println!("extra_candies: {extra_candies}, candies: {:?}, kids_with_candies: {:?}",
             candies, kids_with_candies(candies.clone(), extra_candies));

    let candies = vec![12, 1, 12];
    let extra_candies = 10;
    println!("extra_candies: {extra_candies}, candies: {:?}, kids_with_candies: {:?}",
             candies, kids_with_candies(candies.clone(), extra_candies));
}