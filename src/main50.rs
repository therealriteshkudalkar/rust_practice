fn longest_common_prefix(strs: Vec<String>) -> String {
    // Find the smallest string among these
    let mut prefix = String::new();
    if strs.len() == 0 {
        return prefix;
    }
    let mut str_chars: Vec<Vec<char>> = Vec::new();
    let mut smallest_string_len = usize::MAX;
    for str in &strs {
        str_chars.push(str.chars().collect());
        if str.len() < smallest_string_len {
            smallest_string_len = str.len();
        }
    }
    for index in 0..smallest_string_len {
        // get the first character
        let curr_char = str_chars[0][index];
        let mut all_match = true;
        for str_char in &str_chars {
            if curr_char != str_char[index] {
                all_match = false;
            }
        }
        if !all_match {
            break;
        }
        prefix.push(curr_char);
    }
    return prefix;
}

pub fn main50() {
    let str_vec = vec![String::from("flower"), String::from("flow"), String::from("flight")];
    println!("str_vec: {:?}; prefix: {:?}", str_vec.clone(), longest_common_prefix(str_vec));

    let str_vec = vec![String::from("dog"), String::from("car"), String::from("racer")];
    println!("str_vec: {:?}; prefix: {:?}", str_vec.clone(), longest_common_prefix(str_vec));

    let str_vec = vec![String::from("man"), String::from("manipulative"), String::from("mandarin")];
    println!("str_vec: {:?}; prefix: {:?}", str_vec.clone(), longest_common_prefix(str_vec));
}