
fn find_factors_of_self(str: &str) -> Vec<&str> {
    let mut str_factors: Vec<&str> = Vec::new();
    for (index, _) in str.char_indices() {
        let slice = & str[..=index];
        if is_a_factor(&str, slice) {
            str_factors.push(slice);
        }
    }
    return str_factors;
}

fn is_a_factor(str1: &str, str2: &str) -> bool {
    // is str2 a factor of str1
    if str1.len() % str2.len() != 0  {
        return false;
    }
    let times = str1.len() / str2.len();
    let mut str1_iter = str1.chars();
    for i in 0..times {
        for ch_str2 in str2.chars() {
            let ch_str1 = str1_iter.next().unwrap();
            if ch_str2 != ch_str1 {
                return false;
            }
        }
    }
    return true;
}

fn gcd_of_strings(str1: String, str2: String) -> String {
    // find the factors of str1 & str2
    let mut str1_factors: Vec<&str> = find_factors_of_self(&str1);
    let mut str2_factors: Vec<&str> = find_factors_of_self(&str2);

    for i in (0..str1_factors.len()).rev() {
        for j in (0..str2_factors.len()).rev() {
            if str1_factors[i] == str2_factors[j] {
                return str1_factors[i].to_string();
            }
        }
    }
    return String::from("");
}

pub fn main25() {
    let s1 = String::from("abababab");
    let s2 = String::from("abab");
    println!("str1: {s1}, str2: {s2}, gcd: {}", gcd_of_strings(s1.clone(), s2.clone()));

    let s1 = String::from("ababab");
    let s2 = String::from("ab");
    println!("str1: {s1}, str2: {s2}, gcd: {}", gcd_of_strings(s1.clone(), s2.clone()));

    let s1 = String::from("leet");
    let s2 = String::from("code");
    println!("str1: {s1}, str2: {s2}, gcd: {}", gcd_of_strings(s1.clone(), s2.clone()));
}