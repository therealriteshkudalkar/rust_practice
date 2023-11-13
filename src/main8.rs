fn count_bits(n: i32) -> Vec<i32> {
    if n == 0 {
        return vec![0];
    } else if n == 1 {
        return vec![0, 1];
    }
    let mut ar = vec![0; (n + 1) as usize];
    ar[0] = 0;
    ar[1] = 1;
    for i in 2..n + 1 {
        ar[i as usize] = i % 2 + ar[(i / 2) as usize];
    }
    return ar;
}

pub fn main8() {
    let n = 8;
    println!("n: {n}, count_bits: {:?}", count_bits(n));
}