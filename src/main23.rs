
pub fn reverse_words(s: &String) -> String {
    let mut rev_string = String::new();
    let mut vec_stack: Vec<String> = Vec::new();
    let mut word = String::new();
    for (index, ch) in s.trim().chars().enumerate() {
        if ch == ' ' || index == s.len() - 1 {
            if ch != ' ' {
                word.push(ch);
            }
            // then a word has completed to append that word to the vec_stack
            vec_stack.push(word.clone());
            word.clear();
            continue;
        }
        word.push(ch);
    }
    while !vec_stack.is_empty() {
        // pop each element from the stack and push it
        rev_string.push_str(&*vec_stack.pop().unwrap());
        rev_string.push(' ');
    }
    return rev_string;
}

pub fn main23() {
    let s = String::from("the sky is blue");
    println!("s: {s}, reverse_words: {}", reverse_words(&s));
    let s = String::from("  hello world  ");
    println!("s: {s}, reverse_words: {}", reverse_words(&s));
}