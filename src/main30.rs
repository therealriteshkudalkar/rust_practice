fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut k = 0;
    for i in 0..nums.len() {
        if nums[i] != val {
            nums[k] = nums[i];
            k += 1;
        }
    }
    return k as i32;
}

pub fn main30() {
    let mut vec1 = vec![3,2,2,3];
    let val = 3;
    print!("val: {val}; vec1: {:?}; ", vec1);
    let k = remove_element(&mut vec1, val);
    println!("k: {k}; vec1: {:?}", vec1);

    let mut vec1 = vec![0,1,2,2,3,0,4,2];
    let val = 2;
    print!("val: {val}; vec1: {:?}; ", vec1);
    let k = remove_element(&mut vec1, val);
    println!("k: {k}; vec1: {:?}", vec1);
}