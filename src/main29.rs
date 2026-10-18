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
