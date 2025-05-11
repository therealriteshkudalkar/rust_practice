
pub fn is_match_rec(s: &Vec<char>, p: &Vec<char>, s_index: usize, p_index: usize) -> bool {
    if s_index > s.len() || p_index > p.len() {
        return false;
    } else if s_index == s.len() && p_index == p.len() {
        return true;
    } else if (s_index == s.len() && p_index != p.len()) || (s_index != s.len() && p_index == p.len()) {
        return false;
    }
    let s_char = s[s_index];
    let p_char = p[p_index];
    if s_char == p_char || p_char == '?' {
        return is_match_rec(s, p, s_index + 1, p_index + 1);
    } else if p_char == '*' {
        return is_match_rec(s, p, s_index + 1, p_index) || 
            is_match_rec(s, p, s_index + 1, p_index + 1) ||
            is_match_rec(s, p, s_index, p_index + 1);
        ;
    }
    false
}

pub fn is_match(s: String, p: String) -> bool {
    let s_chars: Vec<char> = s.chars().collect();
    let p_chars: Vec<char> = p.chars().collect();
    is_match_rec(&s_chars, &p_chars, 0, 0)
}

pub fn main69() {
    let str = String::from("aa");
    let pattern = String::from("a?");
    println!("s: {str}; p: {pattern}; {:?}", is_match(str.clone(), pattern.clone()));

    let str = String::from("ab");
    let pattern = String::from("a?");
    println!("s: {str}; p: {pattern}; {:?}", is_match(str.clone(), pattern.clone()));

    let str = String::from("aa");
    let pattern = String::from("*");
    println!("s: {str}; p: {pattern}; {:?}", is_match(str.clone(), pattern.clone()));

    let str = String::from("cb");
    let pattern = String::from("?a");
    println!("s: {str}; p: {pattern}; {:?}", is_match(str.clone(), pattern.clone()));

    let str = String::from("adceb");
    let pattern = String::from("*a*b");
    println!("s: {str}; p: {pattern}; {:?}", is_match(str.clone(), pattern.clone()));

    let str = String::from("");
    let pattern = String::from("*****");
    println!("s: {str}; p: {pattern}; {:?}", is_match(str.clone(), pattern.clone()));
}