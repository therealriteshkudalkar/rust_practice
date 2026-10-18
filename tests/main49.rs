//! Tests for `src/main49.rs` — Roman to Integer.

mod main49 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main49.rs");

    #[cfg(test)]
    mod tests {
        use super::roman_to_int;

        #[test]
        fn additive_numerals() {
            assert_eq!(3, roman_to_int("III".to_string()));
            assert_eq!(58, roman_to_int("LVIII".to_string()));
        }

        #[test]
        fn subtractive_numerals() {
            assert_eq!(4, roman_to_int("IV".to_string()));
            assert_eq!(9, roman_to_int("IX".to_string()));
            assert_eq!(40, roman_to_int("XL".to_string()));
            assert_eq!(90, roman_to_int("XC".to_string()));
            assert_eq!(400, roman_to_int("CD".to_string()));
            assert_eq!(900, roman_to_int("CM".to_string()));
        }

        #[test]
        fn combined_numerals() {
            assert_eq!(39, roman_to_int("XXXIX".to_string()));
            assert_eq!(49, roman_to_int("XLIX".to_string()));
            assert_eq!(99, roman_to_int("XCIX".to_string()));
            assert_eq!(444, roman_to_int("CDXLIV".to_string()));
        }

        #[test]
        fn larger_combined_numerals() {
            assert_eq!(1994, roman_to_int("MCMXCIV".to_string()));
            assert_eq!(2024, roman_to_int("MMXXIV".to_string()));
        }

        #[test]
        fn single_symbol() {
            assert_eq!(1, roman_to_int("I".to_string()));
            assert_eq!(1000, roman_to_int("M".to_string()));
        }

        #[test]
        fn empty_string_is_zero() {
            assert_eq!(0, roman_to_int(String::new()));
        }
    }
}
