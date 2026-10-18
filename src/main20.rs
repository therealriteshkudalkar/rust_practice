pub fn is_vowel(ch: char) -> bool {
    ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u' || ch == 'A' ||
        ch == 'E' || ch == 'I' || ch == 'O' || ch == 'U'
}

pub fn reverse_vowels(s: &String) -> String {
    let mut rev_str = String::new();
    let mut stack: Vec<char> = Vec::new();
    for ch in s.chars() {
        if is_vowel(ch) {
            stack.push(ch);
        }
    }
    for ch in s.chars() {
        if is_vowel(ch) {
            rev_str.push(stack.pop().unwrap());
        } else {
            rev_str.push(ch);
        }
    }
    rev_str
}
