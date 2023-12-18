fn max_product_difference(nums: &Vec<i32>) -> i32 {
    // find maximum difference find the first two largest number and two smallest numbers
    let mut new_nums = nums.clone();
    new_nums.sort();
    let large1: i32 = new_nums[new_nums.len() - 1];
    let large2: i32 = new_nums[new_nums.len() - 2];
    let small1: i32 = new_nums[1];
    let small2: i32 = new_nums[0];
    return large1 * large2 - small1 * small2;
}

pub fn main27() {
    let vec = vec![5, 6, 2, 7, 4];
    println!("vec: {:?}, max_product_difference: {}", vec, max_product_difference(&vec));

    let vec = vec![4, 2, 5, 9, 7, 4, 8];
    println!("vec: {:?}, max_product_difference: {}", vec, max_product_difference(&vec));

    let vec = vec![1, 1, 1, 1];
    println!("vec: {:?}, max_product_difference: {}", &vec, max_product_difference(&vec));
}