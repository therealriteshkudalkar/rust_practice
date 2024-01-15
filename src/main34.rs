fn rotate(nums: &mut Vec<i32>, k: i32) {
    let len = nums.len();
    for _ in 0..k {
        let last_element = nums[len - 1];
        for i in (0..len - 1).rev() {
            nums[i + 1] = nums[i];
        }
        nums[0] = last_element;
    }
}

fn rotate2(nums: &mut Vec<i32>, k: i32) {
    if nums.len() <= 1  || k == nums.len() as i32 {
        return;
    }
    for _ in 0..k {
        let last_num = nums.pop().unwrap();
        nums.insert(0, last_num);
    }
}

fn rotate3(nums: &mut Vec<i32>, k: i32) {
    if k > 0 {
        let len = nums.len();
        if len <= 1 {
            return;
        }
        let k = (k as usize) % len;
        if k == 0 {
            return;
        }
        nums.reverse();
        nums[..k].reverse();
        nums[k..].reverse();
    }
}

pub fn main34() {
    let mut vec = vec![1, 2, 3, 4, 5, 6, 7];
    let k = 3;
    print!("vec: {:?}; k: {k}; ", vec);
    rotate2(&mut vec, k);
    println!("rotated_vec: {:?}", vec);
}