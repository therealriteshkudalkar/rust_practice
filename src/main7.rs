fn climb_stairs(n: i32) -> i32 {
    if n == 0 || n == 1 {
        return 1;
    }
    let mut ar = vec![0; (n + 1) as usize];
    ar[0] = 1;
    ar[1] = 1;
    for i in 2..n + 1 {
        ar[i as usize] = ar[(i - 1) as usize] + ar[(i - 2) as usize];
    }
    return ar[n as usize];
}

pub fn main7() {
    let n = 4;
    println!("n: {n}, climb_stairs: {}", climb_stairs(n));
}