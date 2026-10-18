
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
