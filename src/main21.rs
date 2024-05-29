use std::collections::HashMap;

fn count_chars(s: &String) -> HashMap<char, u32> {
    let mut hm: HashMap<char, u32> = HashMap::new();
    for ch in s.chars() {
        if hm.contains_key(&ch) {
            hm.insert(ch, *hm.get(&ch).unwrap() + 1);
        } else {
            hm.insert(ch, 1);
        }
    }
    return hm;
}

fn is_anagram(s: String, t: String) -> bool {
    let hm_s: HashMap<char, u32> = count_chars(&s);
    let hm_t: HashMap<char, u32> = count_chars(&t);
    // check if both maps have same keys and values
    return hm_s.eq(&hm_t);
}

pub fn main21() {
    let s = String::from("anagram");
    let t = String::from("naagram");
    println!("s: {s}, t: {t}, is_anagram: {}", is_anagram(s.clone(), t.clone()));

    let s = String::from("rat");
    let t = String::from("car");
    println!("s: {s}, t: {t}, is_anagram: {}", is_anagram(s.clone(), t.clone()));
}