fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    // first pass: find the correct position of each element
    // one pointer on nums1 the other on nums2
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut res_index: usize = 0;
    let mut resultant_vec = vec![0; nums1.len()];
    while res_index < (m + n) as usize {
        if i < m as usize && j < n as usize {
            if nums1[i] <= nums2[j] {
                resultant_vec[res_index] = nums1[i];
                i += 1;
            } else {
                resultant_vec[res_index] = nums2[j];
                j += 1;
            }
        } else if i < m as usize && j >= n as usize {
            resultant_vec[res_index] = nums1[i];
            i += 1;
        } else if i >= m as usize && j <= n as usize {
            resultant_vec[res_index] = nums2[j];
            j += 1;
        }
        res_index += 1;
    }
    for (index, num) in resultant_vec.iter().enumerate() {
        nums1[index] = *num;
    }
}

pub fn main29() {
    let mut vec1 = vec![1, 2, 3, 0, 0, 0];
    let mut vec2 = vec![2, 5, 6];
    print!("vec1: {:?}; vec2: {:?}; ", vec1, vec2);
    merge(&mut vec1, 3, &mut vec2, 3);
    println!("merged: {:?}", vec1);

    let mut vec1 = vec![1];
    let mut vec2 = vec![];
    print!("vec1: {:?}; vec2: {:?}; ", vec1, vec2);
    merge(&mut vec1, 1, &mut vec2, 0);
    println!("merged: {:?}", vec1);

    let mut vec1 = vec![0];
    let mut vec2 = vec![1];
    print!("vec1: {:?}; vec2: {:?}; ", vec1, vec2);
    merge(&mut vec1, 0, &mut vec2, 1);
    println!("merged: {:?}", vec1);
}