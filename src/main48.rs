fn reverse(x: i32) -> i32 {
    if x == i32::MIN || x == i32::MAX {
        // cannot calculate absolute as it causes panic
        // it's reverse also doesn't fit in i32 so return 0
        return 0;
    }
    // extract the absolute form
    let mut temp = x.abs();
    // now reverse the digits by checking for overflow
    let mut num: i32 = 0;
    let mut overflow: bool;
    while temp != 0 {
        let rem = temp % 10;
        (num, overflow) = num.overflowing_mul(10);
        if overflow {
            return 0;
        }
        (num, overflow) = num.overflowing_add(rem);
        if overflow {
            return 0;
        }
        temp /= 10;
    }
    if x < 0 {
        (num, overflow) = num.overflowing_mul(-1);
        if overflow {
            return 0;
        }
    }
    return num;
}

pub fn main48() {
    let x = 123;
    println!("x: {x}; reverse: {}", reverse(x));

    let x = -123;
    println!("x: {x}; reverse: {}", reverse(x));

    let x = 120;
    println!("x: {x}; reverse: {}", reverse(x));

    let x = -1534236469;
    println!("x: {x}; reverse: {}", reverse(x));

    let x = 1534236469;
    println!("x: {x}; reverse: {}", reverse(x));

    let x = i32::MIN;
    println!("x: {x}; reverse: {}", reverse(x));

    let x = i32::MAX;
    println!("x: {x}; reverse: {}", reverse(x));
}