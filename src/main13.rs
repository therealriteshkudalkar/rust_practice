
fn tribonacci(n: i32) -> i32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else if n == 2 {
        return 1;
    }
    let mut a = 0;
    let mut b = 1;
    let mut c = 1;
    let mut d = a + b + c;
    let mut count = 3;
    while count != n {
        a = b;
        b = c;
        c = d;
        d = a + b + c;
        count += 1;
    }
    return d;
}

pub fn main13() {
    let n = 37;
    println!("n: {n}; tribonacci: {}", tribonacci(n));
}
