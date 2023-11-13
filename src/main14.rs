
pub fn get_row(row_index: i32) -> Vec<i32> {
    let mut ith_vec = vec![1];
    for i in 1..=row_index {
        let mut curr_vec = vec![0; (i + 1) as usize];
        // make vector
        curr_vec[0] = 1;
        curr_vec[i as usize] = 1;
        for j in 1..=i/2 {
            let val = ith_vec[(j - 1) as usize] + ith_vec[j as usize];
            curr_vec[j as usize] = val;
            curr_vec[(i - j) as usize] = val;
        }
        ith_vec = curr_vec;
    }
    return ith_vec;
}

pub fn main14() {
    let n = 10;
    println!("n: {n}; get_row: {:?}", get_row(n));
}