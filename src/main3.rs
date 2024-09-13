fn is_opening_brace(c: char) -> bool {
    c == '(' || c == '{' || c == '['
}

fn is_closing_brace(c: char) -> bool {
    c == ')' || c == '}' || c == ']'
}

fn are_matching_brace(c1: char, c2: char) -> bool {
    c1 == '(' && c2 == ')' || c1 == '[' && c2 == ']' || c1 == '{' && c2 == '}'
}

fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = Vec::with_capacity(s.len());
    for ch in s.chars() {
        let is_stack_empty = stack.is_empty();
        if is_stack_empty && is_opening_brace(ch) {
            stack.push(ch);
        } else if is_stack_empty && is_closing_brace(ch) {
            return false;
        } else if !is_stack_empty && is_opening_brace(ch) {
            stack.push(ch);
        } else if !is_stack_empty && is_closing_brace(ch) {
            // pop element from the stack
            let popped = stack.pop().unwrap();
            if is_closing_brace(popped) {
                return false;
            } else if is_opening_brace(popped) {
                if are_matching_brace(popped, ch) {
                    continue;
                } else {
                    return false;
                }
            }
        }
    }
    stack.is_empty()
}

pub fn main3() {
    let s = String::from("()");
    println!("s: {s}, is_valid: {}", is_valid(s.clone()));
    let s = String::from("(()");
    println!("s: {s}, is_valid: {}", is_valid(s.clone()));
    let s = String::from("()[]{}");
    println!("s: {s}, is_valid: {}", is_valid(s.clone()));
    let s = String::from("(]");
    println!("s: {s}, is_valid: {}", is_valid(s.clone()));
}
