use std::collections::HashMap;

fn roman_to_int(s: String) -> i32 {
    let mut roman_numeral_symbols: HashMap<char, i32> = HashMap::new();
    roman_numeral_symbols.insert('M', 1000);
    roman_numeral_symbols.insert('D', 500);
    roman_numeral_symbols.insert('C', 100);
    roman_numeral_symbols.insert('L', 50);
    roman_numeral_symbols.insert('X', 10);
    roman_numeral_symbols.insert('V', 5);
    roman_numeral_symbols.insert('I', 1);

    let mut sum = 0;
    let mut prev_char: Option<char> = None;
    for character in s.chars() {
        match prev_char {
            None => {
                if roman_numeral_symbols.contains_key(&character) {
                    // Get the value and add it to the
                    let val = *roman_numeral_symbols.get(&character).unwrap();
                    sum += val;
                } else {
                    return -1;
                }
            }
            Some(prev_ch) => {
                if prev_ch == 'I' {
                    if character == 'X' {
                        sum += 8;
                    } else if character == 'V' {
                        sum += 3;
                    } else {
                        // choose the character from the map
                        sum += *roman_numeral_symbols.get(&character).unwrap();
                    }
                } else if prev_ch == 'X' {
                    if character == 'C' {
                        sum += 80;
                    } else if character == 'L' {
                        sum += 30;
                    } else {
                        // choose the character from the map
                        sum += *roman_numeral_symbols.get(&character).unwrap();
                    }
                } else if prev_ch == 'C' {
                    if character == 'M' {
                        sum += 800;
                    } else if character == 'D' {
                        sum += 300;
                    } else {
                        // choose the character from the map
                        sum += *roman_numeral_symbols.get(&character).unwrap();
                    }
                } else {
                    // choose the character form the map
                    sum += *roman_numeral_symbols.get(&character).unwrap();
                }
            }
        }
        prev_char = Some(character);
    }
    return sum;
}

pub fn main49() {
    let s = String::from("III");
    println!("roman: {s}; integer: {}", roman_to_int(s.clone()));

    let s = String::from("IV");
    println!("roman: {s}; integer: {}", roman_to_int(s.clone()));

    let s = String::from("IX");
    println!("roman: {s}; integer: {}", roman_to_int(s.clone()));

    let s = String::from("XXXIX");
    println!("roman: {s}; integer: {}", roman_to_int(s.clone()));

    let s = String::from("XLIX");
    println!("roman: {s}; integer: {}", roman_to_int(s.clone()));

    let s = String::from("XCIX");
    println!("roman: {s}; integer: {}", roman_to_int(s.clone()));

    let s = String::from("LVIII");
    println!("roman: {s}; integer: {}", roman_to_int(s.clone()));

    let s = String::from("MCMXCIV");
    println!("roman: {s}; integer: {}", roman_to_int(s.clone()));
}