
fn my_pow(x: f64, n: i32) -> f64 {
    return if n == 0 {
        1.0
    } else {
        let mut n = n;
        let mut x = x;
        let mut res = 1.0;
        while n != 0 {
            if n % 2 != 0 {
                res = if n > 0 {
                    res * x
                } else {
                    res / x
                }
            }
            x = x * x;
            n = n / 2;
        }
        res
    }
}

pub fn main62() {
    let x = 2.0;
    let n = 10;
    assert_eq!(1024.0, my_pow(x, n));

    let x = 2.1;
    let n = 3;
    assert_eq!(9.261000000000001, my_pow(x, n));

    let x = 2.0;
    let n = -2;
    assert_eq!(0.250, my_pow(x, n));
}
