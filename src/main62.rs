
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
