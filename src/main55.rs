fn max_depth(s: String) -> i32 {
    if s.len() == 0 {
        return 0;
    } else if s.len() == 1 {
        return 0;
    }
    let mut stack: Vec<char> = Vec::new();
    let mut depth = 0;

    for char in s.chars() {
        if char == '(' {
            // push it in stack and get depth
            stack.push(')');
            let stack_len = stack.len();
            if stack_len > depth {
                depth = stack_len;
            }
        } else if char == ')' {
            // pop from the stack
            let _ = stack.pop();
        }
    }

    return depth as i32;
}

pub fn main55() {
    let s = String::from("(1+(2*3)+((8)/4))+1");
    println!("s: {s}; max_depth: {}", max_depth(s.clone()));

    let s = String::from("(1)+((2))+(((3)))");
    println!("s: {s}; max_depth: {}", max_depth(s.clone()));

    let s = String::from("");
    println!("s: {s}; max_depth: {}", max_depth(s.clone()));

    let s = String::from("()");
    println!("s: {s}; max_depth: {}", max_depth(s.clone()));

    let s = String::from("()()");
    println!("s: {s}; max_depth: {}", max_depth(s.clone()));
}