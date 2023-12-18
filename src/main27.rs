fn max_product_difference(nums: &Vec<i32>) -> i32 {
    // find maximum difference find the first two largest number and two smallest numbers
    let mut large1: i32 = i32::MIN;
    let mut large2: i32 = i32::MIN;
    let mut small1: i32 = i32::MAX;
    let mut small2: i32 = i32::MAX;
    for num in nums {
        if *num >= large2 {
            if *num >= large1 {
                large2 = large1;
                large1 = *num;
            } else {
                large2 = *num;
            }
        }
        if *num <= small2 {
            if *num <= small1 {
                small2 = small1;
                small1 = *num;
            } else {
                small2 = *num;
            }
        }
    }
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