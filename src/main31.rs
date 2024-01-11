fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut k = 1;
    let mut prev_element = nums[0];
    for i in 1..nums.len() {
        if nums[i] != prev_element {
            prev_element = nums[i];
            nums[k] = nums[i];
            k += 1;
        }
    }
    return k as i32;
}

pub fn main31() {
    let mut vec1 = vec![1, 1, 2];
    print!("vec1: {:?}; ", vec1);
    let k = remove_duplicates(&mut vec1);
    println!("k: {k}; vec1: {:?}", vec1);

    let mut vec1 = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    print!("vec1: {:?}; ", vec1);
    let k = remove_duplicates(&mut vec1);
    println!("k: {k}; vec1: {:?}", vec1);
}