
/*
Approach:
create a new string
check what is the smaller string
until the lowest length append characters from word1 to index * 2
until the lowest length append the characters from word2 to index * 2 + 1
after this check which word is larger word
*/

fn merge_alternately(word1: String, word2: String) -> String {
    let mut final_string = vec![' '; word1.len() + word2.len()];
    let (small_word, large_word) = if word1.len() > word2.len() {
        (&word2, &word1)
    } else {
        (&word1, &word2)
    };
    let mut index = 0;
    for ch in word1.chars() {
        if index >= small_word.len() {
            break;
        }
        final_string[index * 2] = ch;
        index += 1;
    }
    let mut index = 0;
    for ch in word2.chars() {
        if index >= small_word.len() {
            break;
        }
        final_string[index * 2 + 1] = ch;
        index += 1;
    }
    let slice = &large_word[small_word.len()..];
    let mut index = small_word.len() * 2;
    for ch in slice.chars() {
        final_string[index] = ch;
        index += 1;
    }

    return final_string.iter().collect();
}

pub fn main24() {
    let s1 = String::from("hello");
    let s2 = String::from("abc");
    println!("s1: {s1}, s2: {s2}, merge_alternatively: {}", merge_alternately(s1.clone(), s2.clone()));
    let s1 = String::from("dude");
    let s2 = String::from("mental");
    println!("s1: {s1}, s2: {s2}, merge_alternatively: {}", merge_alternately(s1.clone(), s2.clone()));
}