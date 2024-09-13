
fn fib(n: i32) -> i32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }
    let mut a = 0;
    let mut b = 1;
    let mut c = a + b;
    let mut count = 2;
    while count != n {
        a = b;
        b = c;
        c = a + b;
        count += 1;
    }
    c
}

pub fn main10() {
    let n = 45;
    println!("n: {n}; fib: {}", fib(n));
}