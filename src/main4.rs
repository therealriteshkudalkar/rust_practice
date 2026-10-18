use std::collections::{HashMap, HashSet};

fn has_equal_frequency(char_map: &HashMap<char, i8>) -> bool {
    let mut freq_map: HashSet<i8> = HashSet::new();
    for (_key, value) in char_map.iter() {
        freq_map.insert(*value);
    }
    freq_map.len() == 1
}

fn equal_frequency(word: String) -> bool {
    let mut char_map: HashMap<char, i8> = HashMap::new();
    for ch in word.chars() {
        if char_map.contains_key(&ch) {
            let count = char_map.get(&ch).unwrap();
            char_map.insert(ch, count + 1_i8);
        } else {
            char_map.insert(ch, 1);
        }
    }

    for ch in word.chars() {
        let mut curr_char_count = 0_i8;
        if char_map.contains_key(&ch) {
            curr_char_count = *char_map.get(&ch).unwrap();
        }
        if curr_char_count - 1 == 0 {
            char_map.remove(&ch);
        } else {
            char_map.insert(ch, curr_char_count - 1);
        }
        // check if it has equal frequency
        if has_equal_frequency(&char_map) {
            return true;
        }
        // alter the map again
        char_map.insert(ch, curr_char_count);
    }
    false
}
