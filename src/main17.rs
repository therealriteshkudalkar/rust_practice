
fn is_subsequence(s: &String, t: &String) -> bool {
    if s.is_empty() {
        return true;
    }
    let byte_s = s.as_bytes();
    let mut index_in_s = 0;
    for (_, ch) in t.char_indices() {
        // only helpful in case the string contains only ascii characters
        let curr_ch_in_s = byte_s[index_in_s] as char;
        if curr_ch_in_s == ch  {
            index_in_s += 1;
            if index_in_s == s.len() {
                return true;
            }
        }
    }
    false
}
