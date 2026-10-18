use std::collections::{HashMap, HashSet};

fn distinct_names_optimized(ideas: Vec<String>) -> i64 {
    // Create a hashset of string from ideas
    let mut count = 0;
    let n = ideas.len();

    // Map {key -> prefix : value -> Set{suffix}}
    let mut prefix_suffix_map = HashMap::new();
    for i in 0..n {
        let prefix = &ideas[i][0..=0];
        let suffix = &ideas[i][1..];
        match prefix_suffix_map.get_mut(prefix) {
            None => {
                let mut new_char_count_map = HashSet::new();
                new_char_count_map.insert(suffix);
                prefix_suffix_map.insert(prefix, new_char_count_map);
            }
            Some(char_count_map) => {
                char_count_map.insert(suffix);
            }
        }
    }

    // Loop through all the prefixes (at most these can be 26)
    let keys: Vec<&&str> = prefix_suffix_map.keys().collect();
    let n = keys.len();
    for i in 0..n {
        let first_group_suffix = keys[i];
        let first_group_set = prefix_suffix_map.get(first_group_suffix).unwrap();
        for j in i + 1..n {
            let second_group_suffix = keys[j];
            let second_group_set = prefix_suffix_map.get(second_group_suffix).unwrap();

            // Calculate the difference between the number of keys in both
            let keys_unique_to_first_map = first_group_set.difference(second_group_set).count();
            let keys_unique_to_second_map = second_group_set.difference(first_group_set).count();

            count += (2 * keys_unique_to_first_map * keys_unique_to_second_map) as i64;
        }
    }
    return count;
}

fn distinct_names(ideas: Vec<String>) -> i64 {
    // Create a hashset of string from ideas
    let set: HashSet<String> = ideas.clone().into_iter().collect();
    let mut count = 0;
    let n = ideas.len();
    for i in 0..n {
        let first_string = &ideas[i];
        let mut first_string_chars = first_string.chars();
        let first_string_first_char = String::from(first_string_chars.nth(0).unwrap());
        let first_string_rest: String = first_string_chars.collect();

        for j in i + 1..n {
            // Check if by exchanging the first letter of the
            let second_string = &ideas[j];
            let mut second_string_chars = second_string.chars();
            let second_string_first_char = String::from(second_string_chars.nth(0).unwrap());
            let second_string_rest: String = second_string_chars.collect();
            if !set.contains(&(first_string_first_char.clone() + &*second_string_rest))
                && !set.contains(&(second_string_first_char + &*first_string_rest))
            {
                // Do a plus one if both are
                count = count + 2;
            }
        }
    }
    return count;
}
