// 9. Palindrome Number
fn is_palindrome(x: i32) -> bool {
    let mut y = 0;
    let mut temp = x.abs();
    while temp != 0 {
        let rem = temp % 10;
        y = y * 10 + rem;
        temp /= 10;
    }
    y == x
}
