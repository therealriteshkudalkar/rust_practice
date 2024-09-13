use std::collections::HashMap;

fn count_vowel_strings_for_character(map: &mut HashMap<String, i32>, vow_ar: &Vec<char>, index: usize, n: i32) -> i32 {
    if n == 0 {
        return 1;
    }
    let v = format!("{}, {n}", vow_ar[index]);
    if map.contains_key(&*v) {
        return *map.get(&*v).unwrap();
    }
    let mut count = 0;
    for i in index..vow_ar.len() {
        count += count_vowel_strings_for_character(map, vow_ar, i, n - 1);
    }
    map.insert(v , count);
    count
}

fn count_vowel_strings(n: i32) -> i32 {
    // a, e, i, o, u
    // aaa, aae, aai, aao, aau, aee, aei, aeo, aeu, eee, eei, eeu, eeo, eii, eio, eiu,
    let mut map: HashMap<String, i32> = HashMap::new();
    let vow_ar = vec!['a', 'e', 'i', 'o', 'u'];
    count_vowel_strings_for_character(&mut map, &vow_ar, 0, n)
}

pub fn main18() {
    let n = 1;
    println!("n: {n}, count_vowel_strings: {}", count_vowel_strings(n));
    let n = 2;
    println!("n: {n}, count_vowel_strings: {}", count_vowel_strings(n));
    let n = 33;
    println!("n: {n}, count_vowel_strings: {}", count_vowel_strings(n));
}