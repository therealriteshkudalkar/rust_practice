
fn reverse_words(s: &String) -> String {
    let splits: Vec<&str> = s.split(" ").collect();
    let mut string = String::new();
    for &split in splits.iter().rev() {
        if split.len() == 0 {
            continue;
        }
        string.push_str(split);
        string.push(' ');
    }
    string.pop();
    return string;
}

pub fn main23() {
    let s = String::from("the sky is blue");
    println!("s: {s}, reverse_words: {}", reverse_words(&s));
    let s = String::from("  hello world  ");
    println!("s: {s}, reverse_words: {}", reverse_words(&s));
    let s = String::from("a good   example");
    println!("s: {s}, reverse_words: {}", reverse_words(&s));
}